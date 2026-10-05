use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database query failed: {0}")]
    SqliteError(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct SqliteStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStore {
    pub fn new_in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init_tables()?;
        Ok(store)
    }

    pub fn new_file(path: &str) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let store = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        store.init_tables()?;
        Ok(store)
    }

    fn init_tables(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS agents (
                agent_id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                host_platform TEXT NOT NULL,
                scopes_json TEXT NOT NULL,
                registered_at INTEGER NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS approval_gates (
                approval_id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                action_description TEXT NOT NULL,
                risk_level INTEGER NOT NULL,
                context_payload_json TEXT NOT NULL,
                status INTEGER NOT NULL,
                reviewer_id TEXT,
                rationale TEXT,
                created_at INTEGER NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                entry_id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                payload_hash TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS agent_messages (
                message_id TEXT PRIMARY KEY,
                sender_agent_id TEXT NOT NULL,
                target_agent_id TEXT NOT NULL,
                session_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            );",
            [],
        )?;

        Ok(())
    }

    pub fn register_agent(
        &self,
        agent_id: &str,
        name: &str,
        host_platform: &str,
        scopes_json: &str,
        timestamp: i64,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO agents (agent_id, name, host_platform, scopes_json, registered_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![agent_id, name, host_platform, scopes_json, timestamp],
        )?;
        Ok(())
    }

    pub fn create_approval_request(
        &self,
        approval_id: &str,
        agent_id: &str,
        action_description: &str,
        risk_level: i32,
        context_payload_json: &str,
        status: i32,
        timestamp: i64,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO approval_gates (approval_id, agent_id, action_description, risk_level, context_payload_json, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![approval_id, agent_id, action_description, risk_level, context_payload_json, status, timestamp],
        )?;
        Ok(())
    }

    pub fn resolve_approval_request(
        &self,
        approval_id: &str,
        reviewer_id: &str,
        status: i32,
        rationale: &str,
    ) -> Result<bool, DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "UPDATE approval_gates SET status = ?1, reviewer_id = ?2, rationale = ?3 WHERE approval_id = ?4",
            params![status, reviewer_id, rationale, approval_id],
        )?;
        Ok(rows > 0)
    }

    pub fn get_approval_request(&self, approval_id: &str) -> Result<Option<(i32, Option<String>, Option<String>)>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT status, reviewer_id, rationale FROM approval_gates WHERE approval_id = ?1")?;
        let mut rows = stmt.query(params![approval_id])?;

        if let Some(row) = rows.next()? {
            let status: i32 = row.get(0)?;
            let reviewer_id: Option<String> = row.get(1)?;
            let rationale: Option<String> = row.get(2)?;
            Ok(Some((status, reviewer_id, rationale)))
        } else {
            Ok(None)
        }
    }

    pub fn record_audit_entry(
        &self,
        entry_id: &str,
        agent_id: &str,
        event_type: &str,
        payload_json: &str,
        timestamp: i64,
    ) -> Result<(), DbError> {
        let mut hasher = Sha256::new();
        hasher.update(payload_json.as_bytes());
        let payload_hash = format!("{:x}", hasher.finalize());

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO audit_logs (entry_id, agent_id, event_type, payload_hash, payload_json, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![entry_id, agent_id, event_type, payload_hash, payload_json, timestamp],
        )?;
        Ok(())
    }

    pub fn query_audit_logs(&self, agent_id: &str, start_timestamp: i64, limit: i64) -> Result<Vec<(String, String, String, String, String, i64)>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT entry_id, agent_id, event_type, payload_hash, payload_json, timestamp
             FROM audit_logs WHERE agent_id = ?1 AND timestamp >= ?2 ORDER BY timestamp DESC LIMIT ?3"
        )?;

        let rows = stmt.query_map(params![agent_id, start_timestamp, limit], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })?;

        let mut entries = Vec::new();
        for r in rows {
            entries.push(r?);
        }
        Ok(entries)
    }

    pub fn record_agent_message(
        &self,
        message_id: &str,
        sender_agent_id: &str,
        target_agent_id: &str,
        session_id: &str,
        payload_json: &str,
        timestamp: i64,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO agent_messages (message_id, sender_agent_id, target_agent_id, session_id, payload_json, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![message_id, sender_agent_id, target_agent_id, session_id, payload_json, timestamp],
        )?;
        Ok(())
    }
}