use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Database error: {0}")]
    DbError(String),
}

#[derive(Clone)]
pub struct ContextStore {
    // In-memory lineage graph store (Simulates Kùzu / Neo4j embedded instance)
    nodes: Arc<Mutex<Vec<GraphNode>>>,
    edges: Arc<Mutex<Vec<GraphEdge>>>,
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub relationship: String,
}

impl ContextStore {
    pub fn new() -> Result<Self, StoreError> {
        Ok(Self {
            nodes: Arc::new(Mutex::new(Vec::new())),
            edges: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn record_stage_execution(
        &self,
        pipeline_id: &str,
        stage_id: &str,
        attestation_hash: &str,
    ) -> Result<(), StoreError> {
        let mut nodes = self.nodes.lock().unwrap();
        let mut edges = self.edges.lock().unwrap();

        let stage_node_id = format!("Stage:{}:{}", pipeline_id, stage_id);
        let attestation_node_id = format!("Attestation:{}", attestation_hash);

        nodes.push(GraphNode {
            id: stage_node_id.clone(),
            label: "StageExecution".into(),
            properties: serde_json::json!({ "pipeline": pipeline_id, "stage": stage_id }),
        });

        nodes.push(GraphNode {
            id: attestation_node_id.clone(),
            label: "DSSEAttestation".into(),
            properties: serde_json::json!({ "hash": attestation_hash }),
        });

        edges.push(GraphEdge {
            from: stage_node_id,
            to: attestation_node_id,
            relationship: "PRODUCED_EVIDENCE",
        });

        Ok(())
    }

    pub fn get_lineage_count(&self) -> usize {
        self.edges.lock().unwrap().len()
    }
}