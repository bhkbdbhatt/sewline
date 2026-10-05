use crate::db::sqlite::SqliteStore;
use crate::grpc::service::pb::agent_governance_service_server::AgentGovernanceServiceServer;
use crate::grpc::service::AgentGovernanceServiceImpl;
use tonic::transport::Server;

pub mod db;
pub mod grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "[::1]:50051".parse()?;
    let store = SqliteStore::new_in_memory()?;
    let service = AgentGovernanceServiceImpl::new(store);

    tracing::info!("Sewline Agent Governance Control Plane listening on {}", addr);

    Server::builder()
        .add_service(AgentGovernanceServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}