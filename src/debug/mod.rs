//! Debug Module - Runtime debugging with NDM visibility
//!
//! This module provides debugging utilities for ALN development,
//! including NDM score visibility and session tracing.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// ALN debugger for runtime visibility
pub struct AlnDebugger {
    session_id: Option<String>,
    verbose: bool,
}

/// Debug session info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSession {
    pub session_id: String,
    pub ndm_score: f64,
    pub ndm_state: String,
    pub active_capabilities: Vec<String>,
    pub recent_events: Vec<DebugEvent>,
    pub timestamp: i64,
}

/// Debug event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugEvent {
    pub event_type: String,
    pub description: String,
    pub ndm_impact: f64,
    pub timestamp: i64,
}

/// NDM visibility report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NdmReport {
    pub current_score: f64,
    pub state: String,
    pub suspicion_triggers: Vec<String>,
    pub recent_changes: Vec<NdmChange>,
}

/// NDM change record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NdmChange {
    pub from_score: f64,
    pub to_score: f64,
    pub trigger: String,
    pub timestamp: i64,
}

impl AlnDebugger {
    /// Create a new debugger
    pub fn new() -> Self {
        Self {
            session_id: None,
            verbose: false,
        }
    }

    /// Set session ID for debugging
    pub fn with_session(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    /// Enable verbose output
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Get debug session info
    pub fn get_session_info(&self) -> Result<DebugSession, crate::error::DevToolsError> {
        let session_id = self.session_id.clone().unwrap_or_else(|| "default".to_string());
        
        // In production, query actual session state
        Ok(DebugSession {
            session_id,
            ndm_score: 0.2,
            ndm_state: "Normal".to_string(),
            active_capabilities: vec!["NETCLIENT".to_string()],
            recent_events: vec![],
            timestamp: Utc::now().timestamp(),
        })
    }

    /// Get NDM report
    pub fn get_ndm_report(&self) -> Result<NdmReport, crate::error::DevToolsError> {
        Ok(NdmReport {
            current_score: 0.2,
            state: "Normal".to_string(),
            suspicion_triggers: vec![],
            recent_changes: vec![],
        })
    }

    /// Trace capability usage
    pub fn trace_capability(&self, capability: &str) -> Result<DebugEvent, crate::error::DevToolsError> {
        Ok(DebugEvent {
            event_type: "capability_use".to_string(),
            description: format!("Capability {} used", capability),
            ndm_impact: 0.0,
            timestamp: Utc::now().timestamp(),
        })
    }
}

impl Default for AlnDebugger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_creation() {
        let debugger = AlnDebugger::new();
        assert!(debugger.session_id.is_none());
    }

    #[test]
    fn test_session_info() {
        let debugger = AlnDebugger::new().with_session("test-123");
        let info = debugger.get_session_info();
        assert!(info.is_ok());
    }
}
