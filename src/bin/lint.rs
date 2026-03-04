//! ALN Lint CLI - Command-line linting tool

use aln_dev_tools::{AlnLinter, LintSeverity};
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aln-lint")]
#[command(about = "ALN shard linting with capability/effect checking", long_about = None)]
struct Cli {
    /// Files or directories to lint
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Enable strict mode (warnings become errors)
    #[arg(short, long)]
    strict: bool,

    /// Output format (text, json, sarif)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Disable specific rules (comma-separated)
    #[arg(short, long)]
    disable_rules: Option<String>,

    /// Show rule explanations
    #[arg(long)]
    explain: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    let mut linter = AlnLinter::new().with_strict_mode(cli.strict);

    for path in &cli.paths {
        if path.is_file() {
            let result = linter.lint_file(&path.to_string_lossy())?;
            print_result(&result, &cli.format)?;
        } else if path.is_dir() {
            // Lint all ALN files in directory
            for entry in walkdir::WalkDir::new(path) {
                let entry = entry?;
                if entry.path().extension().map_or(false, |ext| ext == "aln") {
                    let result = linter.lint_file(&entry.path().to_string_lossy())?;
                    print_result(&result, &cli.format)?;
                }
            }
        }
    }

    Ok(())
}

fn print_result(result: &aln_dev_tools::LintResult, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(result)?);
        }
        "text" => {
            println!("\n{}", "Lint Results".bold().blue());
            println!("File: {}", result.file_path);
            println!("Errors: {}, Warnings: {}, Info: {}", 
                result.total_errors.to_string().red(),
                result.total_warnings.to_string().yellow(),
                result.total_info.to_string().cyan());
            println!("Status: {}", if result.passed { "PASS".green() } else { "FAIL".red() });

            for issue in &result.issues {
                let severity = match issue.severity {
                    LintSeverity::Error => "ERROR".red(),
                    LintSeverity::Warning => "WARNING".yellow(),
                    LintSeverity::Info => "INFO".cyan(),
                    LintSeverity::Hint => "HINT".white(),
                };
                println!("  [{}] {}: {}", severity, issue.rule_id, issue.message);
                if let Some(ref suggestion) = issue.suggestion {
                    println!("    → {}", suggestion.dimmed());
                }
            }
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(result)?);
        }
    }

    Ok(())
}
