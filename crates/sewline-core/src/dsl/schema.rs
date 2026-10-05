use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineDefinition {
    pub name: String,
    pub version: String,
    pub compliance_profile: String, // e.g., "DO-178C-DAL-A"
    pub metadata: HashMap<String, String>,
    pub stages: Vec<StageDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageDefinition {
    pub id: String,
    pub name: String,
    pub adapter: String,
    pub depends_on: Vec<String>,
    pub retry_policy: Option<RetryPolicy>,
    pub params: HashMap<String, serde_json::Value>,
    pub gates: Vec<GateDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GateDefinition {
    pub id: String,
    pub name: String,
    pub policy_rule: String, // OPA/Rego rule query, e.g. "sewline.parasoft.zero_violations"
    pub policy_wasm: Option<Vec<u8>>, // Embedded WASM binary for air-gapped Rego execution
    pub enforce: bool,
}