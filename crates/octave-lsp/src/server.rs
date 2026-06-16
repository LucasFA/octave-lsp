//! Octave LSP server: handles LSP protocol over stdio.

use std::collections::HashMap;

use anyhow::Result;
use lsp_server::{Connection, Message, Notification};
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument, Notification as _};
use lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    InitializeParams, Position, PublishDiagnosticsParams, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};
use parser::parse;

/// Runs the LSP server on stdin/stdout.
pub fn run() -> Result<()> {
    let (connection, io_threads) = Connection::stdio();

    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        ..ServerCapabilities::default()
    };

    let init_params: InitializeParams =
        serde_json::from_value(connection.initialize(serde_json::to_value(capabilities)?)?)?;

    tracing_init(&init_params);

    let mut documents: HashMap<String, String> = HashMap::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
            }
            Message::Notification(not) => {
                handle_notification(&connection, &not, &mut documents)?;
            }
            Message::Response(_) => {}
        }
    }

    io_threads.join()?;
    Ok(())
}

fn tracing_init(_init_params: &InitializeParams) {}

fn handle_notification(
    connection: &Connection,
    not: &Notification,
    documents: &mut HashMap<String, String>,
) -> Result<()> {
    match not.method.as_str() {
        DidOpenTextDocument::METHOD => {
            let params: DidOpenTextDocumentParams =
                serde_json::from_value(not.params.clone())?;
            let uri = params.text_document.uri;
            let text = params.text_document.text;
            let key = uri.to_string();
            documents.insert(key, text.clone());
            publish_diagnostics(connection, &uri, &text);
        }
        DidChangeTextDocument::METHOD => {
            let params: DidChangeTextDocumentParams =
                serde_json::from_value(not.params.clone())?;
            let uri = params.text_document.uri;
            let key = uri.to_string();
            let mut text = documents.remove(&key).unwrap_or_default();
            for change in &params.content_changes {
                text.clone_from(&change.text);
            }
            documents.insert(key, text.clone());
            publish_diagnostics(connection, &uri, &text);
        }
        _ => {}
    }
    Ok(())
}

fn publish_diagnostics(connection: &Connection, uri: &Uri, text: &str) {
    let parse = parse(text);
    let syntax = parse.syntax();
    let validation_errors = ast::validation::validate(&syntax);

    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    for error in parse.errors() {
        diagnostics.push(parse_error_to_diagnostic(error, text));
    }

    for error in &validation_errors {
        diagnostics.push(validation_error_to_diagnostic(error, text));
    }

    let params = PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics,
        version: None,
    };

    let _ = connection.sender.send(Message::Notification(Notification {
        method: "textDocument/publishDiagnostics".to_string(),
        params: serde_json::to_value(params).unwrap(),
    }));
}

fn offset_to_position(text: &str, offset: u32) -> Position {
    let offset = offset as usize;
    let mut line = 0u32;
    let mut character = 0u32;

    for ch in text[..offset.min(text.len())].chars() {
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
    }

    Position { line, character }
}

fn parse_error_to_diagnostic(error: &parser::ParseError, text: &str) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: offset_to_position(text, u32::from(error.range.start())),
            end: offset_to_position(text, u32::from(error.range.end())),
        },
        severity: Some(DiagnosticSeverity::ERROR),
        message: error.to_string(),
        ..Diagnostic::default()
    }
}

fn validation_error_to_diagnostic(
    error: &ast::validation::ValidationError,
    text: &str,
) -> Diagnostic {
    let range = error.range();
    Diagnostic {
        range: Range {
            start: offset_to_position(text, u32::from(range.start())),
            end: offset_to_position(text, u32::from(range.end())),
        },
        severity: Some(DiagnosticSeverity::ERROR),
        message: error.to_string(),
        ..Diagnostic::default()
    }
}
