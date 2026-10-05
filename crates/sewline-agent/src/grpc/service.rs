use crate::db::sqlite::SqliteStore;
use chrono::Utc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod pb {
    tonic::include_proto!("sewline.agent.v1");
}

use pb::agent_governance_service_server::AgentGovernanceService;
use pb::*;

pub struct AgentGovernanceServiceImpl {
    db: SqliteStore,
}

impl AgentGovernanceServiceImpl {
    pub fn new(db: SqliteStore) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl AgentGovernanceService for AgentGovernanceServiceImpl {
    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<RegisterAgentResponse>, Status> {
        let req = request.into_inner();
        let timestamp = Utc::now().timestamp();
        let token = format!("sat_{}", Uuid::new_v4().simple());

        let scopes_json = serde_json::to_string(&req.scopes)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        self.db
            .register_agent(&req.agent_id, &req.name, &req.host_platform, &scopes_json, timestamp)
            .map_err(|e| Status::internal(e.to_string()))?;

        let audit_payload = serde_json::json!({
            "action": "REGISTER_AGENT",
            "scopes": req.scopes,
            "platform": req.host_platform
        }).to_string();

        self.db
            .record_audit_entry(&Uuid::new_v4().to_string(), &req.agent_id, "AGENT_REGISTERED", &audit_payload, timestamp)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RegisterAgentResponse {
            registered: true,
            token,
            registered_at: timestamp,
        }))
    }

    async fn request_approval(
        &self,
        request: Request<RequestApprovalRequest>,
    ) -> Result<Response<RequestApprovalResponse>, Status> {
        let req = request.into_inner();
        let approval_id = format!("appr_{}", Uuid::new_v4().simple());
        let timestamp = Utc::now().timestamp();

        // Low risk actions automatically pass, Medium/High/Critical require human approval
        let initial_status = if req.risk_level == RiskLevel::Low as i32 {
            ApprovalStatus::Approved as i32
        } else {
            ApprovalStatus::Pending as i32
        };

        self.db
            .create_approval_request(
                &approval_id,
                &req.agent_id,
                &req.action_description,
                req.risk_level,
                &req.context_payload_json,
                initial_status,
                timestamp,
            )
            .map_err(|e| Status::internal(e.to_string()))?;

        let audit_payload = serde_json::json!({
            "approval_id": approval_id,
            "risk_level": req.risk_level,
            "action": req.action_description
        }).to_string();

        self.db
            .record_audit_entry(&Uuid::new_v4().to_string(), &req.agent_id, "APPROVAL_REQUESTED", &audit_payload, timestamp)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RequestApprovalResponse {
            approval_id,
            status: initial_status,
        }))
    }

    async fn check_approval(
        &self,
        request: Request<CheckApprovalRequest>,
    ) -> Result<Response<CheckApprovalResponse>, Status> {
        let req = request.into_inner();

        let result = self
            .db
            .get_approval_request(&req.approval_id)
            .map_err(|e| Status::internal(e.to_string()))?;

        match result {
            Some((status, reviewer_id, rationale)) => Ok(Response::new(CheckApprovalResponse {
                approval_id: req.approval_id,
                status,
                reviewer_id: reviewer_id.unwrap_or_default(),
                rationale: rationale.unwrap_or_default(),
            })),
            None => Err(Status::not_found("Approval ID not found")),
        }
    }

    async fn resolve_approval(
        &self,
        request: Request<ResolveApprovalRequest>,
    ) -> Result<Response<ResolveApprovalResponse>, Status> {
        let req = request.into_inner();
        let timestamp = Utc::now().timestamp();

        let new_status = if req.approve {
            ApprovalStatus::Approved as i32
        } else {
            ApprovalStatus::Rejected as i32
        };

        let success = self
            .db
            .resolve_approval_request(&req.approval_id, &req.reviewer_id, new_status, &req.rationale)
            .map_err(|e| Status::internal(e.to_string()))?;

        if success {
            let audit_payload = serde_json::json!({
                "approval_id": req.approval_id,
                "reviewer": req.reviewer_id,
                "approved": req.approve,
                "rationale": req.rationale
            }).to_string();

            self.db
                .record_audit_entry(&Uuid::new_v4().to_string(), &req.reviewer_id, "APPROVAL_RESOLVED", &audit_payload, timestamp)
                .map_err(|e| Status::internal(e.to_string()))?;
        }

        Ok(Response::new(ResolveApprovalResponse {
            success,
            new_status,
        }))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        let message_id = format!("msg_{}", Uuid::new_v4().simple());
        let timestamp = Utc::now().timestamp();

        self.db
            .record_agent_message(
                &message_id,
                &req.sender_agent_id,
                &req.target_agent_id,
                &req.session_id,
                &req.payload_json,
                timestamp,
            )
            .map_err(|e| Status::internal(e.to_string()))?;

        let audit_payload = serde_json::json!({
            "message_id": message_id,
            "target_agent": req.target_agent_id,
            "session_id": req.session_id
        }).to_string();

        self.db
            .record_audit_entry(&Uuid::new_v4().to_string(), &req.sender_agent_id, "MESSAGE_SENT", &audit_payload, timestamp)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SendMessageResponse {
            message_id,
            timestamp,
        }))
    }

    async fn query_audit_trail(
        &self,
        request: Request<AuditQueryRequest>,
    ) -> Result<Response<AuditQueryResponse>, Status> {
        let req = request.into_inner();

        let logs = self
            .db
            .query_audit_logs(&req.agent_id, req.start_timestamp, req.limit)
            .map_err(|e| Status::internal(e.to_string()))?;

        let entries = logs
            .into_iter()
            .map(|(entry_id, agent_id, event_type, payload_hash, payload_json, timestamp)| AuditLogEntry {
                entry_id,
                agent_id,
                event_type,
                payload_hash,
                payload_json,
                timestamp,
            })
            .collect();

        Ok(Response::new(AuditQueryResponse { entries }))
    }
}