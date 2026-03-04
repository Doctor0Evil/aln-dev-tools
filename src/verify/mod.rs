//! Envelope Verifier - Zes-encryption verification for Sourzes
//!
//! This module verifies zes-encrypted envelopes, signatures, and
//! hex-stamps before deployment.

use crate::error::DevToolsError;
use zes_crypto_lib::{ZesEnvelope, EnvelopeConfig};
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// ALN envelope verifier
pub struct AlnVerifier {
    strict_mode: bool,
    verify_signatures: bool,
    verify_hex_stamp: bool,
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub file_path: String,
    pub is_valid: bool,
    pub signature_valid: bool,
    pub hex_stamp_valid: bool,
    pub envelope_version: String,
    pub timestamp: i64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl AlnVerifier {
    /// Create a new verifier
    pub fn new() -> Self {
        Self {
            strict_mode: true,
            verify_signatures: true,
            verify_hex_stamp: true,
        }
    }

    /// Enable/disable strict mode
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Verify a file
    pub fn verify_file(&self, path: &str) -> Result<VerificationResult, DevToolsError> {
        let content = std::fs::read(path)?;
        self.verify_content(&content, path)
    }

    /// Verify content bytes
    pub fn verify_content(&self, content: &[u8], path: &str) -> Result<VerificationResult, DevToolsError> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Parse envelope
        let envelope = match ZesEnvelope::deserialize(content) {
            Ok(e) => e,
            Err(e) => {
                errors.push(format!("Failed to parse envelope: {}", e));
                return Ok(VerificationResult {
                    file_path: path.to_string(),
                    is_valid: false,
                    signature_valid: false,
                    hex_stamp_valid: false,
                    envelope_version: "unknown".to_string(),
                    timestamp: Utc::now().timestamp(),
                    errors,
                    warnings,
                });
            }
        };

        // Verify signatures
        let signature_valid = if self.verify_signatures {
            match envelope.verify_signatures() {
                Ok(_) => true,
                Err(e) => {
                    errors.push(format!("Signature verification failed: {}", e));
                    false
                }
            }
        } else {
            true
        };

        // Verify hex-stamp
        let hex_stamp_valid = if self.verify_hex_stamp {
            match envelope.verify_hex_stamp() {
                Ok(_) => true,
                Err(e) => {
                    errors.push(format!("Hex-stamp verification failed: {}", e));
                    false
                }
            }
        } else {
            true
        };

        let is_valid = signature_valid && hex_stamp_valid && errors.is_empty();

        Ok(VerificationResult {
            file_path: path.to_string(),
            is_valid,
            signature_valid,
            hex_stamp_valid,
            envelope_version: envelope.version,
            timestamp: envelope.timestamp,
            errors,
            warnings,
        })
    }

    /// Batch verify multiple files
    pub fn verify_batch(&self, paths: &[&str]) -> Result<Vec<VerificationResult>, DevToolsError> {
        let mut results = Vec::new();
        
        for path in paths {
            let result = self.verify_file(path)?;
            results.push(result);
        }

        Ok(results)
    }
}

impl Default for AlnVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let verifier = AlnVerifier::new();
        assert!(verifier.strict_mode);
    }
}
