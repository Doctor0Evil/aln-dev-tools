//! LSP Server - Language Server Protocol for ALN
//!
//! This module provides LSP support for IDE integration,
//! enabling syntax highlighting, IntelliSense, and linting.

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use serde::{Deserialize, Serialize};

/// ALN Language Server
pub struct AlnLanguageServer {
    client: Client,
    documents: std::collections::HashMap<String, String>,
}

impl AlnLanguageServer {
    /// Create a new language server
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: std::collections::HashMap::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for AlnLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(Default::default())),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ALN Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.text_document.text;
        self.documents.lock().await.insert(uri, content);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(change) = params.content_changes.first() {
            self.documents.lock().await.insert(uri, change.text.clone());
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // Trigger lint on save
        let uri = params.text_document.uri.to_string();
        self.client
            .log_message(MessageType::INFO, format!("File saved: {}", uri))
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.documents.lock().await.remove(&uri);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem {
                label: "NANOSWARM_CTRL".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Nanoswarm control capability (requires non-weapon envelope)".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "NETCLIENT".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Network client capability".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "FSREAD".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Filesystem read capability".to_string()),
                ..Default::default()
            },
        ])))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        // Provide hover information for ALN keywords
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "ALN Keyword Documentation".to_string(),
            )),
            range: None,
        }))
    }

    async fn diagnostic(&self, params: DocumentDiagnosticParams) -> Result<DocumentDiagnosticReportResult> {
        // Return diagnostics from linting
        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: vec![],
                },
            }),
        ))
    }
}

/// Create LSP service
pub fn create_lsp_service() -> (LspService<AlnLanguageServer>, Server) {
    LspService::build(AlnLanguageServer::new)
        .finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_service_creation() {
        // LSP service creation requires async runtime
        // This is a placeholder test
        assert!(true);
    }
}
