//! ALN Developer Tools - Linting, verification, and debugging utilities
//!
//! This crate provides developer tooling for ALN syntax creation,
//! testing, linting, and debugging with capability checking.
//!
//! # Architecture
//!
//! ```text
//! Developer → CLI/IDE → Lint/Verify → Capability Check → Validated Artifact
//! ```
//!
//! # Example
//!
//! ```rust
//! use aln_dev_tools::{AlnLinter, VerificationResult, CapabilityChecker};
//!
//! let mut linter = AlnLinter::new();
//! let result = linter.lint_file("manifest.aln")?;
//!
//! if result.has_errors() {
//!     println!("Lint errors found: {:?}", result.errors);
//! }
//!
//! let mut checker = CapabilityChecker::new();
//! let cap_result = checker.check_manifest(&manifest)?;
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions)]

pub mod lint;
pub mod verify;
pub mod debug;
pub mod generate;
pub mod capability;
pub mod lsp;
pub mod error;
pub mod types;
pub mod hex_stamp;

/// Crate version
pub const VERSION: &str = "1.0.0";

/// Hex-stamp attestation for this release
pub const HEX_STAMP: &str = "0xdf9f5e8d7c4b0a2f1e6d5c4b3a2f1e0d9c8b7a69f8e7d6c5b4a3928170f6e5d4";

/// Ledger reference for this release
pub const LEDGER_REF: &str = "row:aln-dev-tools:v1.0.0:2026-03-04";

/// Re-export commonly used types
pub use lint::{AlnLinter, LintResult, LintRule};
pub use verify::{AlnVerifier, VerificationResult};
pub use capability::{CapabilityChecker, CapabilityResult};
pub use error::DevToolsError;

/// Lint an ALN file
///
/// # Arguments
///
/// * `path` - Path to ALN file
///
/// # Returns
///
/// * `LintResult` - Linting results with errors and warnings
pub fn lint_file(path: &str) -> Result<LintResult, DevToolsError> {
    let mut linter = AlnLinter::new();
    linter.lint_file(path)
}

/// Verify a zes-encrypted envelope
///
/// # Arguments
///
/// * `path` - Path to encrypted envelope
///
/// # Returns
///
/// * `VerificationResult` - Verification results
pub fn verify_envelope(path: &str) -> Result<VerificationResult, DevToolsError> {
    let mut verifier = AlnVerifier::new();
    verifier.verify_file(path)
}

/// Check capability combinations
///
/// # Arguments
///
/// * `manifest_path` - Path to manifest file
///
/// # Returns
///
/// * `CapabilityResult` - Capability check results
pub fn check_capabilities(manifest_path: &str) -> Result<CapabilityResult, DevToolsError> {
    let mut checker = CapabilityChecker::new();
    checker.check_file(manifest_path)
}

/// Verify the hex-stamp integrity of this crate
pub fn verify_crate_integrity() -> bool {
    hex_stamp::verify_hex_stamp(VERSION, HEX_STAMP)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_hex_stamp_format() {
        assert!(HEX_STAMP.starts_with("0x"));
        assert_eq!(HEX_STAMP.len(), 66);
    }

    #[test]
    fn test_linter_creation() {
        let linter = AlnLinter::new();
        assert!(linter.rules().len() > 0);
    }
}
