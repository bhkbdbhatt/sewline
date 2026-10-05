pub mod adapter;
pub mod dsl;
pub mod executor;
pub mod gate;
pub mod store;

#[cfg(test)]
mod tests {
    use super::*;
    use adapter::github_actions::GitHubActionsAdapter;
    use adapter::jama::JamaAdapter;
    use adapter::parasoft::ParasoftAdapter;
    use dsl::schema::PipelineDefinition;
    use ed25519_dalek::SigningKey;
    use executor::engine::WorkflowEngine;
    use gate::attestation::DsseSigner;
    use gate::evaluator::GateEngine;
    use rand::rngs::OsRng;
    use std::sync::Arc;
    use store::graph::ContextStore;

    #[tokio::test]
    async fn test_end_to_end_do178c_pipeline_execution() {
        // 1. Setup Crypographic Signing Key for DSSE Evidence Attestation
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let signer = DsseSigner::new(signing_key, "key-sewline-master-01".into());

        // 2. Initialize Engine, Gate Evaluator and Graph Context Store
        let gate_engine = GateEngine::new(signer);
        let context_store = ContextStore::new().expect("Failed to initialize Context Store");
        let mut engine = WorkflowEngine::new(gate_engine, context_store.clone());

        // 3. Register Plugin Adapters
        engine.register_adapter(Arc::new(GitHubActionsAdapter::new()));
        engine.register_adapter(Arc::new(JamaAdapter::new()));
        engine.register_adapter(Arc::new(ParasoftAdapter::new()));

        // 4. Parse YAML Pipeline Definition DSL
        let yaml_dsl = include_str!("../../../pipelines/do178c_sample_pipeline.yaml");
        let pipeline: PipelineDefinition =
            serde_yaml::from_str(yaml_dsl).expect("Failed to parse Pipeline DSL YAML");

        // 5. Execute Pipeline Engine
        let attestations = engine
            .execute_pipeline(pipeline)
            .await
            .expect("Pipeline execution failed");

        // 6. Verification
        assert_eq!(attestations.len(), 2, "Expected 2 signed DSSE attestations for the enforced gates");
        assert_eq!(attestations[0].payload_type, "application/vnd.in-toto+json");
        assert!(!attestations[0].signatures.is_empty());
        assert_eq!(context_store.get_lineage_count(), 2, "Expected 2 graph lineage edge records");
    }
}