# ALN Developer Tools

**Developer tooling for ALN syntax creation, testing, linting, and debugging with capability checking**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aln-dev-tools.svg)](https://crates.io/crates/aln-dev-tools)
[![Docs](https://docs.rs/aln-dev-tools/badge.svg)](https://docs.rs/aln-dev-tools)
[![Hex-Stamp](https://img.shields.io/badge/hex--stamp-0xdf9f5e8d7c4b0a2f1e6d5c4b3a2f1e0d9c8b7a69-green.svg)](docs/security/hex-stamp-attestation.md)
[![Audit Status](https://img.shields.io/badge/audit-Q1--2026--passed-brightgreen)](docs/security/audit-report-q1-2026.md)

## Purpose

`aln-dev-tools` is the **developer experience layer** for the ALN Sovereign Stack. It provides CLI tools, IDE extensions, and linting utilities that catch capability violations, security issues, and governance problems at development time—before artifacts reach production.

This guarantees:
- **Early Violation Detection** - Capability issues caught before deployment
- **IDE Integration** - Real-time syntax highlighting and error detection
- **Zes-Envelope Verification** - Verify encryption before deployment
- **NDM Debugging** - Real-time NDM score visibility during development
- **Non-Weaponization Guards** - Prevent accidental weapon-like code patterns

## Architecture

┌─────────────────────────────────────────────────────────────────┐
│ DEVELOPER WORKSTATION │
│ (VS Code / JetBrains / Terminal / CI/CD) │
└────────────────────────────┬────────────────────────────────────┘
│ ALN Files / Sourzes
▼
┌─────────────────────────────────────────────────────────────────┐
│ aln-dev-tools │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ CLI Tools (aln-lint, aln-verify, aln-debug) │ │
│ └───────────────────────────────────────────────────────────┘ │
│ │ │ │ │
│ ▼ ▼ ▼ │
│ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │
│ │IDE Extensions│ │CapabilityCheck│ │ZesVerifier │ │
│ └──────────────┘ └──────────────┘ └──────────────┘ │
│ │ │ │ │
│ └──────────────────┼──────────────────┘ │
│ ▼ │
│ ┌───────────────────────────────────────────────────────────┐ │
│ │ Output: Validated ALN Artifacts + Reports │ │
│ └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ DEPLOYMENT PIPELINE │
│ (sovereigntycore → ROW/RPM → Googolswarm) │
└─────────────────────────────────────────────────────────────────┘


## Key Components

| Component | Description |
|-----------|-------------|
| `aln-lint` | ALN shard linting with capability/effect checking |
| `aln-verify` | Signature and hash verification for Sourzes |
| `aln-debug` | Runtime debugging with NDM visibility |
| `vscode-ext` | VS Code extension for ALN syntax highlighting |
| `jetbrains-plugin` | JetBrains IDE plugin for ALN development |
| `capability-check` | Static analysis for forbidden capability combos |
| `zes-verifier` | Zes-encryption envelope verification |

## CLI Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `aln-lint` | Lint ALN files for violations | `aln-lint manifest.aln` |
| `aln-verify` | Verify zes-envelopes | `aln-verify sourze.zes` |
| `aln-debug` | Debug with NDM visibility | `aln-debug --session <id>` |
| `aln-gen` | Generate ALN schemas from templates | `aln-gen sourze --name my-project` |
| `aln-cap-check` | Check capability combinations | `aln-cap-check manifest.json` |

## Quick Start

```bash
# Clone the repository
git clone https://github.com/aln-sovereign/aln-dev-tools.git
cd aln-dev-tools

# Build all tools
cargo build --release

# Install CLI tools
cargo install --path .

# Lint an ALN manifest
aln-lint my-sourze.aln

# Verify a zes-encrypted envelope
aln-verify my-sourze.zes

# Check capability combinations
aln-cap-check my-manifest.json

# Install VS Code extension
code --install-extension aln-dev-tools.vsix

Non-Weaponization Developer Guardrails
Guardrail,Description,Enforcement
Default Templates,NANOSWARM_CTRL disabled by default,Template system
Capability Warnings,Warn on high-risk capability requests,aln-lint
Forbidden Combos,Block weapon-like capability combinations,aln-cap-check
Effect Typing,Require explicit effect declarations,ALN IR
NDM Awareness,Show NDM impact of capability choices,aln-debug

IDE Integration
IDE,Extension,Features
VS Code,aln-dev-tools.vsix,"Syntax highlighting, linting, IntelliSense"
JetBrains,aln-dev-tools.jar,"Full IDE integration, refactoring"
Vim/Neovim,aln.vim,"Syntax highlighting, LSP support"
Emacs,aln-mode.el,Major mode with completion

Security Properties
Static Analysis - Capability violations caught at compile time
Envelope Verification - Zes-encryption verified before deployment
NDM Visibility - Developers see NDM impact of their code
Non-Weaponization - Accidental weapon-like patterns flagged
Audit Trail - All lint/verify actions logged to ROW/RPM
Governance
All development tools require:
Open Source - All tools publicly auditable (MIT License)
Capability Checking - No bypass of security checks
ROW/RPM Logging - Tool usage logged for audit
Community Maintained - Open contribution with DID anchoring
Hex-Stamp Attestation: 0xdf9f5e8d7c4b0a2f1e6d5c4b3a2f1e0d9c8b7a69f8e7d6c5b4a3928170f6e5d4
Ledger Reference: row:aln-dev-tools:v1.0.0:2026-03-04
Organichain Anchor: org:pending
License
MIT License - See LICENSE for details.
⚠️ Developer Notice: These tools help catch issues early, but final enforcement happens in sovereigntycore. Never bypass linting in production pipelines.
