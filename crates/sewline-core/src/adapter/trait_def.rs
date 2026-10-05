use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Transport error: {0}")]
    NetworkError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionInput {
    pub stage_id: String,
    pub params: HashMap<String, serde_json::Value>,
    pub workspace_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOutput {
    pub success: bool,
    pub exit_code: i32,
    pub metrics: HashMap<String, serde_json::Value>,
    pub artifacts: Vec<ArtifactRef>,
    pub raw_logs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub name: String,
    pub path: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub target_tool: String,
    pub supported_inputs: Vec<String>,
    pub produced_outputs: Vec<String>,
}

#[async_trait]
pub trait StageExecutor: Send + Sync {
    fn manifest(&self) -> AdapterManifest;
    async fn validate_config(&self, params: &HashMap<String, serde_json::Value>) -> Result<(), AdapterError>;
    async fn execute(&self, input: ExecutionInput) -> Result<ExecutionOutput, AdapterError>;
}