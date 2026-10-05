use crate::adapter::trait_def::ExecutionOutput;
use crate::dsl::schema::GateDefinition;
use crate::gate::attestation::{DigestSet, DsseSigner, Envelope, Predicate, Statement, Subject};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GateError {
    #[error("Gate failure: {0}")]
    GateFailed(String),
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
}

pub struct GateEngine {
    signer: DsseSigner,
}

impl GateEngine {
    pub fn new(signer: DsseSigner) -> Self {
        Self { signer }
    }

    pub fn evaluate_and_attest(
        &self,
        gate: &GateDefinition,
        stage_id: &str,
        output: &ExecutionOutput,
    ) -> Result<Envelope, GateError> {
        // Pure Policy-as-Code Rule Evaluation Logic (Rego engine execution abstraction)
        let passed = match gate.policy_rule.as_str() {
            "sewline.parasoft.zero_violations" => {
                let violations = output
                    .metrics
                    .get("violations_total")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(999);
                violations == 0
            }
            "sewline.jama.full_coverage" => {
                let uncovered = output
                    .metrics
                    .get("uncovered_requirements")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(999);
                uncovered == 0
            }
            _ => {
                // Fallback / default rule evaluation
                output.success
            }
        };

        if !passed && gate.enforce {
            return Err(GateError::GateFailed(format!(
                "Gate '{}' failed rule check '{}'",
                gate.name, gate.policy_rule
            )));
        }

        // Generate subject hash from output logs
        let mut hasher = Sha256::new();
        hasher.update(output.raw_logs.as_bytes());
        let log_hash = format!("{:x}", hasher.finalize());

        let statement = Statement {
            _type: "https://in-toto.io/Statement/v0.1".to_string(),
            subject: vec![Subject {
                name: format!("stage-output-{}", stage_id),
                digest: DigestSet { sha256: log_hash },
            }],
            predicate_type: "https://sewline.dev/attestation/v1".to_string(),
            predicate: Predicate {
                stage_id: stage_id.to_string(),
                gate_id: gate.id.clone(),
                passed,
                policy_rule: gate.policy_rule.clone(),
                evaluated_metrics: serde_json::to_value(&output.metrics).unwrap_or_default(),
                timestamp: "2026-10-05T11:24:00Z".to_string(),
            },
        };

        Ok(self.signer.sign_statement(&statement))
    }
}