//! ALN Linter - Syntax and capability linting for ALN files
//!
//! This module provides comprehensive linting for ALN manifests,
//! catching capability violations, security issues, and governance problems.

use crate::error::DevToolsError;
use aln_syntax_core::validator::SchemaValidator;
use serde::{Deserialize, Serialize};
use std::path::Path;
use colored::Colorize;

/// ALN linter for all file types
pub struct AlnLinter {
    rules: Vec<LintRule>,
    validator: SchemaValidator,
    strict_mode: bool,
}

/// Lint rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub severity: LintSeverity,
    pub enabled: bool,
}

/// Lint severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Lint result for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintResult {
    pub file_path: String,
    pub total_errors: usize,
    pub total_warnings: usize,
    pub total_info: usize,
    pub issues: Vec<LintIssue>,
    pub passed: bool,
}

/// Individual lint issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintIssue {
    pub rule_id: String,
    pub severity: LintSeverity,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub suggestion: Option<String>,
}

impl AlnLinter {
    /// Create a new linter with default rules
    pub fn new() -> Self {
        Self {
            rules: Self::default_rules(),
            validator: SchemaValidator::new(),
            strict_mode: false,
        }
    }

    /// Enable strict mode (warnings become errors)
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Get all lint rules
    pub fn rules(&self) -> &[LintRule] {
        &self.rules
    }

    /// Lint a file
    pub fn lint_file(&mut self, path: &str) -> Result<LintResult, DevToolsError> {
        let content = std::fs::read_to_string(path)?;
        self.lint_content(&content, path)
    }

    /// Lint content string
    pub fn lint_content(&mut self, content: &str, path: &str) -> Result<LintResult, DevToolsError> {
        let mut issues = Vec::new();

        // Schema validation
        if let Err(e) = self.validator.validate_aln_content(content) {
            issues.push(LintIssue {
                rule_id: "SCHEMA_VALID".to_string(),
                severity: LintSeverity::Error,
                message: format!("Schema validation failed: {}", e),
                line: None,
                column: None,
                suggestion: Some("Check ALN syntax against canonical schemas".to_string()),
            });
        }

        // Capability checks
        issues.extend(self.check_capabilities(content)?);

        // Non-weaponization checks
        issues.extend(self.check_non_weaponization(content)?);

        // Style checks
        issues.extend(self.check_style(content));

        // Count by severity
        let total_errors = issues.iter().filter(|i| i.severity == LintSeverity::Error).count();
        let total_warnings = issues.iter().filter(|i| i.severity == LintSeverity::Warning).count();
        let total_info = issues.iter().filter(|i| i.severity == LintSeverity::Info).count();

        // In strict mode, warnings become errors
        let effective_errors = if self.strict_mode {
            total_errors + total_warnings
        } else {
            total_errors
        };

        Ok(LintResult {
            file_path: path.to_string(),
            total_errors: effective_errors,
            total_warnings: if self.strict_mode { 0 } else { total_warnings },
            total_info,
            issues,
            passed: effective_errors == 0,
        })
    }

    /// Check capability violations
    fn check_capabilities(&self, content: &str) -> Result<Vec<LintIssue>, DevToolsError> {
        let mut issues = Vec::new();

        // Check for forbidden capability combinations
        if content.contains("NANOSWARM_CTRL") && content.contains("NETSERVER") {
            issues.push(LintIssue {
                rule_id: "CAP_FORBIDDEN_COMBO".to_string(),
                severity: LintSeverity::Error,
                message: "Forbidden capability combination: NANOSWARM_CTRL + NETSERVER".to_string(),
                line: None,
                column: None,
                suggestion: Some("Remove NETSERVER capability or use NetClient instead".to_string()),
            });
        }

        Ok(issues)
    }

    /// Check non-weaponization patterns
    fn check_non_weaponization(&self, content: &str) -> Result<Vec<LintIssue>, DevToolsError> {
        let mut issues = Vec::new();

        // Check for weapon-like function names (heuristic)
        let weapon_patterns = ["weapon", "attack", "damage", "kinetic", "offensive"];
        for pattern in &weapon_patterns {
            if content.to_lowercase().contains(pattern) {
                issues.push(LintIssue {
                    rule_id: "NON_WEAPON_CHECK".to_string(),
                    severity: LintSeverity::Warning,
                    message: format!("Potentially weapon-related term detected: '{}'", pattern),
                    line: None,
                    column: None,
                    suggestion: Some("Ensure this is for ecological/healthcare purposes only".to_string()),
                });
            }
        }

        Ok(issues)
    }

    /// Check code style
    fn check_style(&self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        // Check for trailing whitespace
        for (line_num, line) in content.lines().enumerate() {
            if line.ends_with(' ') || line.ends_with('\t') {
                issues.push(LintIssue {
                    rule_id: "STYLE_TRAILING_WS".to_string(),
                    severity: LintSeverity::Hint,
                    message: "Trailing whitespace detected".to_string(),
                    line: Some(line_num + 1),
                    column: Some(line.len()),
                    suggestion: Some("Remove trailing whitespace".to_string()),
                });
            }
        }

        issues
    }

    /// Get default lint rules
    fn default_rules() -> Vec<LintRule> {
        vec![
            LintRule {
                rule_id: "SCHEMA_VALID".to_string(),
                name: "Schema Validation".to_string(),
                description: "Ensure ALN content matches canonical schemas".to_string(),
                severity: LintSeverity::Error,
                enabled: true,
            },
            LintRule {
                rule_id: "CAP_FORBIDDEN_COMBO".to_string(),
                name: "Forbidden Capability Combinations".to_string(),
                description: "Prevent weapon-like capability combinations".to_string(),
                severity: LintSeverity::Error,
                enabled: true,
            },
            LintRule {
                rule_id: "NON_WEAPON_CHECK".to_string(),
                name: "Non-Weaponization Check".to_string(),
                description: "Flag potentially weapon-related patterns".to_string(),
                severity: LintSeverity::Warning,
                enabled: true,
            },
            LintRule {
                rule_id: "STYLE_TRAILING_WS".to_string(),
                name: "Trailing Whitespace".to_string(),
                description: "Detect trailing whitespace".to_string(),
                severity: LintSeverity::Hint,
                enabled: true,
            },
        ]
    }
}

impl Default for AlnLinter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_creation() {
        let linter = AlnLinter::new();
        assert!(!linter.rules().is_empty());
    }

    #[test]
    fn test_capability_check() {
        let mut linter = AlnLinter::new();
        let content = r#"
            capabilities: ["NANOSWARM_CTRL", "NETSERVER"]
        "#;
        
        let result = linter.lint_content(content, "test.aln").unwrap();
        assert!(!result.passed);
        assert!(result.total_errors > 0);
    }
}
