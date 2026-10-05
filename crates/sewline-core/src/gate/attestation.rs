use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    pub _type: String, // "https://in-toto.io/Statement/v0.1"
    pub subject: Vec<Subject>,
    pub predicate_type: String, // "https://sewline.dev/attestation/v1"
    pub predicate: Predicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub digest: DigestSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestSet {
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Predicate {
    pub stage_id: String,
    pub gate_id: String,
    pub passed: bool,
    pub policy_rule: String,
    pub evaluated_metrics: serde_json::Value,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub payload: String, // Base64 encoded Statement JSON
    pub payload_type: String, // "application/vnd.in-toto+json"
    pub signatures: Vec<SignatureBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBlock {
    pub keyid: String,
    pub sig: String, // Base64 encoded ed25519 signature
}

pub struct DsseSigner {
    signing_key: SigningKey,
    key_id: String,
}

impl DsseSigner {
    pub fn new(signing_key: SigningKey, key_id: String) -> Self {
        Self { signing_key, key_id }
    }

    pub fn sign_statement(&self, statement: &Statement) -> Envelope {
        let json_bytes = serde_json::to_vec(statement).expect("Statement serialization failed");
        let payload_b64 = openssl_base64(&json_bytes);

        // In-Toto PAE (Pre-Authentication Encoding) structure for DSSE:
        // PAE(type, payload) = "DSSEv1" + " " + len(type) + " " + type + " " + len(payload) + " " + payload
        let payload_type = "application/vnd.in-toto+json";
        let pae = format!(
            "DSSEv1 {} {} {} {}",
            payload_type.len(),
            payload_type,
            json_bytes.len(),
            String::from_utf8_lossy(&json_bytes)
        );

        let signature = self.signing_key.sign(pae.as_bytes());
        let sig_b64 = openssl_base64(&signature.to_bytes());

        Envelope {
            payload: payload_b64,
            payload_type: payload_type.to_string(),
            signatures: vec![SignatureBlock {
                keyid: self.key_id.clone(),
                sig: sig_b64,
            }],
        }
    }
}

fn openssl_base64(data: &[u8]) -> String {
    use sha2::digest::geometry::u8;
    // Standard Base64 encoding without external crate dependency beyond std/core
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };

        let triple = (b0 << 16) | (b1 << 8) | b2;

        out.push(CHARSET[((triple >> 18) & 63) as usize] as char);
        out.push(CHARSET[((triple >> 12) & 63) as usize] as char);
        if i + 1 < data.len() {
            out.push(CHARSET[((triple >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(CHARSET[(triple & 63) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}