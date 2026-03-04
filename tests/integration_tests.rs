//! ALN Developer Tools Integration Tests

use aln_dev_tools::{AlnLinter, AlnVerifier, CapabilityChecker, AlnGenerator};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_lint_valid_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("valid.aln");
    
    let content = r#"
manifest_id: test-123
capabilities:
  - NETCLIENT
  - FSREAD
"#;
    
    fs::write(&file_path, content).unwrap();
    
    let mut linter = AlnLinter::new();
    let result = linter.lint_file(&file_path.to_string_lossy()).unwrap();
    
    assert!(result.passed);
    assert_eq!(result.total_errors, 0);
}

#[test]
fn test_lint_invalid_capabilities() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("invalid.aln");
    
    let content = r#"
manifest_id: test-123
capabilities:
  - NANOSWARM_CTRL
  - NETSERVER
"#;
    
    fs::write(&file_path, content).unwrap();
    
    let mut linter = AlnLinter::new();
    let result = linter.lint_file(&file_path.to_string_lossy()).unwrap();
    
    assert!(!result.passed);
    assert!(result.total_errors > 0);
}

#[test]
fn test_capability_checker() {
    let checker = CapabilityChecker::new();
    let content = r#"capabilities: ["NANOSWARM_CTRL", "NETSERVER"]"#;
    
    let result = checker.check_content(content, "test.aln").unwrap();
    
    assert!(!result.passed);
    assert!(!result.violations.is_empty());
}

#[test]
fn test_generator_sourze() {
    let dir = tempdir().unwrap();
    let output_path = dir.path().join("manifest.aln").to_string_lossy().to_string();
    
    let generator = AlnGenerator::new();
    let request = aln_dev_tools::generate::GenerateRequest {
        artifact_type: "sourze".to_string(),
        name: "test-sourze".to_string(),
        capabilities: vec!["NETCLIENT".to_string()],
        output_path,
    };

    let result = generator.generate_sourze(&request).unwrap();
    
    assert!(result.success);
    assert!(!result.generated_files.is_empty());
}

#[test]
fn test_strict_mode() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("warning.aln");
    
    let content = r#"
manifest_id: test-123
capabilities:
  - NETCLIENT
"#;
    
    fs::write(&file_path, content).unwrap();
    
    // Normal mode
    let mut linter = AlnLinter::new().with_strict_mode(false);
    let result_normal = linter.lint_file(&file_path.to_string_lossy()).unwrap();
    
    // Strict mode
    let mut linter = AlnLinter::new().with_strict_mode(true);
    let result_strict = linter.lint_file(&file_path.to_string_lossy()).unwrap();
    
    // In strict mode, warnings count as errors
    assert!(result_strict.total_errors >= result_normal.total_errors);
}

#[test]
fn test_batch_verification() {
    let dir = tempdir().unwrap();
    
    // Create multiple test files
    let mut paths = Vec::new();
    for i in 0..3 {
        let file_path = dir.path().join(format!("test_{}.aln", i));
        let content = format!(r#"manifest_id: test-{}"#, i);
        fs::write(&file_path, content).unwrap();
        paths.push(file_path.to_string_lossy().to_string());
    }

    let verifier = AlnVerifier::new();
    // Note: These will fail validation (not proper envelopes)
    // but test batch processing works
    for path in &paths {
        let result = verifier.verify_file(path);
        // Expected to fail (not valid envelopes)
        assert!(result.is_err() || !result.unwrap().is_valid);
    }
}
