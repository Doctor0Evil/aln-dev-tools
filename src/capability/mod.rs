//! Capability Checker - Static analysis for capability combinations
//!
//! This module checks for forbidden capability combinations at
//! development time, preventing weapon-like patterns.

use crate::error::DevToolsError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Capability checker for manifests
pub struct CapabilityChecker {
    forbidden_combos: Vec<ForbiddenCombo>,
}

/// Forbidden capability combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenCombo {
    pub combo_id: String,
    pub capabilities: Vec<String>,
    pub reason: String,
    pub severity: String,
}

/// Capability check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResult {
    pub file_path: String,
    pub passed: bool,
    pub capabilities_found: Vec<String>,
    pub violations: Vec<CapabilityViolation>,
    pub warnings: Vec<String>,
}

/// Capability violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityViolation {
    pub combo_id: String,
    pub capabilities: Vec<String>,
    pub reason: String,
    pub severity: String,
}

impl CapabilityChecker {
    /// Create a new capability checker
    pub fn new() -> Self {
        Self {
            forbidden_combos: Self::default_forbidden_combos(),
        }
    }

    /// Check a file
    pub fn check_file(&self, path: &str) -> Result<CapabilityResult, DevToolsError> {
        let content = std::fs::read_to_string(path)?;
        self.check_content(&content, path)
    }

    /// Check content string
    pub fn check_content(&self, content: &str, path: &str) -> Result<CapabilityResult, DevToolsError> {
        let mut capabilities = HashSet::new();
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Extract capabilities from content (simplified parsing)
        self.extract_capabilities(content, &mut capabilities);

        // Check against forbidden combinations
        for combo in &self.forbidden_combos {
            let combo_set: HashSet<&String> = combo.capabilities.iter().collect();
            let has_all = combo_set.iter().all(|c| capabilities.contains(*c));

            if has_all {
                violations.push(CapabilityViolation {
                    combo_id: combo.combo_id.clone(),
                    capabilities: combo.capabilities.clone(),
                    reason: combo.reason.clone(),
                    severity: combo.severity.clone(),
                });
            }
        }

        // Check for high-risk capabilities
        let high_risk = ["NANOSWARM_CTRL", "KERNEL_GUARD", "RAW_SOCKET"];
        for cap in &high_risk {
            if capabilities.contains(&cap.to_string()) {
                warnings.push(format!("High-risk capability detected: {}", cap));
            }
        }

        let passed = violations.is_empty();

        Ok(CapabilityResult {
            file_path: path.to_string(),
            passed,
            capabilities_found: capabilities.into_iter().collect(),
            violations,
            warnings,
        })
    }

    /// Extract capabilities from content
    fn extract_capabilities(&self, content: &str, caps: &mut HashSet<String>) {
        // Simplified extraction - in production, use proper ALN parser
        let cap_patterns = [
            "NANOSWARM_CTRL", "NETSERVER", "NETCLIENT", "FSREAD", "FSWRITE",
            "KERNELGUARD", "AICHAT_BRIDGE", "USB_HID", "SERIAL_MCU", "GPU_COMPUTE",
        ];

        for pattern in &cap_patterns {
            if content.contains(pattern) {
                caps.insert(pattern.to_string());
            }
        }
    }

    /// Get default forbidden combinations
    fn default_forbidden_combos() -> Vec<ForbiddenCombo> {
        vec![
            ForbiddenCombo {
                combo_id: "weapon_ctrl_network".to_string(),
                capabilities: vec!["NANOSWARM_CTRL".to_string(), "NETSERVER".to_string()],
                reason: "Prevents remote weaponization of Nanoswarm".to_string(),
                severity: "critical".to_string(),
            },
            ForbiddenCombo {
                combo_id: "ctrl_fs_hardware".to_string(),
                capabilities: vec!["NANOSWARM_CTRL".to_string(), "FSWRITE".to_string(), "USB_HID".to_string()],
                reason: "Prevents hardware takeover via swarm control".to_string(),
                severity: "critical".to_string(),
            },
            ForbiddenCombo {
                combo_id: "ctrl_gpu".to_string(),
                capabilities: vec!["NANOSWARM_CTRL".to_string(), "GPU_COMPUTE".to_string()],
                reason: "Prevents GPU-based attack vector via swarm".to_string(),
                severity: "high".to_string(),
            },
        ]
    }
}

impl Default for CapabilityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_checker() {
        let checker = CapabilityChecker::new();
        let content = r#"capabilities: ["NANOSWARM_CTRL", "NETSERVER"]"#;
        
        let result = checker.check_content(content, "test.aln").unwrap();
        assert!(!result.passed);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_safe_capabilities() {
        let checker = CapabilityChecker::new();
        let content = r#"capabilities: ["NETCLIENT", "FSREAD"]"#;
        
        let result = checker.check_content(content, "test.aln").unwrap();
        assert!(result.passed);
    }
}
