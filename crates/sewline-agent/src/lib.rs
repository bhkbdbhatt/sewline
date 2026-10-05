pub mod db;
pub mod grpc;

#[cfg(test)]
mod tests {
    use crate::db::sqlite::SqliteStore;
    use crate::grpc::service::pb::agent_governance_service_server::AgentGovernanceService;
    use crate::grpc::service::pb::*;
    use crate::grpc::service::AgentGovernanceServiceImpl;
    use tonic::Request;

    #[tokio::test]
    async fn test_agent_registration_and_approval_workflow() {
        let db = SqliteStore::new_in_memory().unwrap();
        let service = AgentGovernanceServiceImpl::new(db);

        // 1. Register Agent
        let reg_req = Request::new(RegisterAgentRequest {
            agent_id: "agent-copilot-01".into(),
            name: "GitHub Copilot Workspace Agent".into(),
            host_platform: "GitHub".into(),
            scopes: vec![AgentScope {
                resource: "repo:flight-core".into(),
                action: ScopeAction::Write as i32,
            }],
        });

        let reg_res = service.register_agent(reg_req).await.unwrap().into_inner();
        assert!(reg_res.registered);
        assert!(!reg_res.token.is_empty());

        // 2. High Risk Action requires Human Approval
        let app_req = Request::new(RequestApprovalRequest {
            agent_id: "agent-copilot-01".into(),
            action_description: "Deploy binary to Flight Control Target".into(),
            risk_level: RiskLevel::High as i32,
            context_payload_json: r#"{"target":"actuator-01"}"#.into(),
        });

        let app_res = service.request_approval(app_req).await.unwrap().into_inner();
        assert_eq!(app_res.status, ApprovalStatus::Pending as i32);

        // 3. Human Resolve Approval
        let resolve_req = Request::new(ResolveApprovalRequest {
            approval_id: app_res.approval_id.clone(),
            reviewer_id: "certifying-engineer-42".into(),
            approve: true,
            rationale: "MISRA analysis clean and tests pass".into(),
        });

        let resolve_res = service.resolve_approval(resolve_req).await.unwrap().into_inner();
        assert!(resolve_res.success);
        assert_eq!(resolve_res.new_status, ApprovalStatus::Approved as i32);

        // 4. Verify Immutable Audit Trail
        let audit_req = Request::new(AuditQueryRequest {
            agent_id: "agent-copilot-01".into(),
            start_timestamp: 0,
            limit: 10,
        });

        let audit_res = service.query_audit_trail(audit_req).await.unwrap().into_inner();
        assert_eq!(audit_res.entries.len(), 2, "Expected registration and approval audit records");
    }
}