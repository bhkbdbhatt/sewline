use crate::adapter::trait_def::{ExecutionInput, StageExecutor};
use crate::dsl::schema::PipelineDefinition;
use crate::gate::attestation::Envelope;
use crate::gate::evaluator::GateEngine;
use crate::store::graph::ContextStore;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Adapter not registered: {0}")]
    AdapterNotFound(String),
    #[error("Stage execution error: {0}")]
    StageFailed(String),
    #[error("Gate failure: {0}")]
    GateFailed(String),
    #[error("Store error: {0}")]
    StoreError(#[from] crate::store::graph::StoreError),
}

pub struct WorkflowEngine {
    adapters: HashMap<String, Arc<dyn StageExecutor>>,
    gate_engine: GateEngine,
    context_store: ContextStore,
}

impl WorkflowEngine {
    pub fn new(gate_engine: GateEngine, context_store: ContextStore) -> Self {
        Self {
            adapters: HashMap::new(),
            gate_engine,
            context_store,
        }
    }

    pub fn register_adapter(&mut self, adapter: Arc<dyn StageExecutor>) {
        let manifest = adapter.manifest();
        self.adapters.insert(manifest.id, adapter);
    }

    pub async fn execute_pipeline(
        &self,
        pipeline: PipelineDefinition,
    ) -> Result<Vec<Envelope>, EngineError> {
        let mut attestations = Vec::new();

        for stage in &pipeline.stages {
            tracing::info!("Executing Stage: {} via {}", stage.id, stage.adapter);

            let adapter = self
                .adapters
                .get(&stage.adapter)
                .ok_or_else(|| EngineError::AdapterNotFound(stage.adapter.clone()))?;

            let input = ExecutionInput {
                stage_id: stage.id.clone(),
                params: stage.params.clone(),
                workspace_path: "/tmp/sewline-workspace".into(),
            };

            let output = adapter
                .execute(input)
                .await
                .map_err(|e| EngineError::StageFailed(e.to_string()))?;

            for gate in &stage.gates {
                let envelope = self
                    .gate_engine
                    .evaluate_and_attest(gate, &stage.id, &output)
                    .map_err(|e| EngineError::GateFailed(e.to_string()))?;

                let attestation_hash = &envelope.signatures[0].sig[..16];
                self.context_store
                    .record_stage_execution(&pipeline.name, &stage.id, attestation_hash)?;

                attestations.push(envelope);
            }
        }

        Ok(attestations)
    }
}