use crate::adapter::trait_def::{AdapterError, AdapterManifest, ExecutionInput, ExecutionOutput, StageExecutor};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;

pub struct GitHubActionsAdapter;

impl GitHubActionsAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl StageExecutor for GitHubActionsAdapter {
    fn manifest(&self) -> AdapterManifest {
        AdapterManifest {
            id: "adapter-github-actions".into(),
            name: "GitHub Actions Orchestrator Adapter".into(),
            version: "0.1.0".into(),
            target_tool: "GitHub Actions".into(),
            supported_inputs: vec!["workflow_id".into(), "ref".into()],
            produced_outputs: vec!["repo_state".into(), "commit_sha".into()],
        }
    }

    async fn validate_config(&self, params: &HashMap<String, serde_json::Value>) -> Result<(), AdapterError> {
        if !params.contains_key("repository") {
            return Err(AdapterError::ConfigError("Missing 'repository' param".into()));
        }
        Ok(())
    }

    async fn execute(&self, input: ExecutionInput) -> Result<ExecutionOutput, AdapterError> {
        self.validate_config(&input.params)?;

        // Stubbed execution simulating triggering/checking workflow status via local mock or API
        let repo = input.params.get("repository").and_then(|v| v.as_str()).unwrap_or("unknown");
        let ref_spec = input.params.get("ref").and_then(|v| v.as_str()).unwrap_or("main");

        let mut metrics = HashMap::new();
        metrics.insert("commit_sha".to_string(), json!("a1b2c3d4e5f67890"));
        metrics.insert("repository".to_string(), json!(repo));
        metrics.insert("ref".to_string(), json!(ref_spec));

        Ok(ExecutionOutput {
            success: true,
            exit_code: 0,
            metrics,
            artifacts: vec![],
            raw_logs: format!("Successfully synchronised repository {} at ref {}", repo, ref_spec),
        })
    }
}