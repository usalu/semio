//! 📡️ Jack language server — JSON-RPC LSP subset over trinity graphs.
//!
//! 🧹️ This crate was a residual not restructured by the trinity plugin's own crate-consolidation
//! migration (ticket `26/08/05/TRINITY-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`), only
//! repointed to keep building; it was first brought under `-D warnings` clippy gating by its own
//! de-sandwich (`26/08/06/TRINITY-PLUGIN-RESIDUAL-MOP-UP-JACK-TOOLS`), which surfaced these pre-existing
//! findings across ~40 call sites of small serde-`Value`-building helpers and one query-backend enum —
//! allowed here rather than hand-touched, matching the `result_large_err`/`large_enum_variant`
//! precedent in `📋️TEMPLATE.md` §9/§12 for findings the verification gate itself creates.
#![allow(clippy::needless_pass_by_value, clippy::large_enum_variant)]

use mathematical_graph_dsl::{complete, format, hover, lint, semantic_tokens, BoardQueryableGraph, Completion, Diagnostic, DiagnosticSeverity, Hover, QueryableGraph};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use trinity::artifacts::jack::Graph;
use trinity::core::{example_graph_fixture, OwnedTrinityQueryableGraph};

//#region ⚠️ Errors
/// ⚠️ Jack language-server fixture-loading errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityJackLspError {
    /// 🧩️ Trinity graph fixture load/validation failure.
    #[error(transparent)]
    Graph(#[from] trinity::artifacts::jack::TrinityRamError),
    /// 🌐️ Board-domain graph fixture load/validation failure.
    #[error(transparent)]
    Dsl(#[from] mathematical_graph_dsl::GraphDslError),
}
//#endregion ⚠️ Errors

// #region 🔖️LspTypes
#[derive(Clone, Debug, Deserialize)]
pub struct JsonRpcMessage {
    #[serde(default)]
    id: Option<Value>,
    method: Option<String>,
    params: Option<Value>,
}

#[derive(Clone, Debug, Deserialize)]
struct TextDocumentItem {
    uri: String,
    version: i64,
    text: String,
}

#[derive(Clone, Debug, Deserialize)]
struct DidOpenParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentItem,
}

#[derive(Clone, Debug, Deserialize)]
struct DidChangeParams {
    #[serde(rename = "textDocument")]
    text_document: VersionedTextDocument,
    #[serde(rename = "contentChanges")]
    content_changes: Vec<ContentChange>,
}

#[derive(Clone, Debug, Deserialize)]
struct VersionedTextDocument {
    uri: String,
    version: i64,
}

#[derive(Clone, Debug, Deserialize)]
struct ContentChange {
    text: String,
}

#[derive(Clone, Debug, Deserialize)]
struct TextDocumentPositionParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentRef,
    position: Position,
}

#[derive(Clone, Debug, Deserialize)]
struct TextDocumentRef {
    uri: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Position {
    line: u32,
    character: u32,
}

#[derive(Clone, Debug, Deserialize)]
struct FormattingParams {
    #[serde(rename = "textDocument")]
    text_document: TextDocumentRef,
}
// #endregion 🔖️LspTypes

// #region 🔖️JackLanguageServer
enum JackGraphBackend {
    Trinity(OwnedTrinityQueryableGraph),
    Board(BoardQueryableGraph),
}

impl JackGraphBackend {
    fn as_queryable(&self) -> &dyn QueryableGraph {
        match self {
            Self::Trinity(graph) => graph,
            Self::Board(board) => board,
        }
    }
}

pub struct JackLanguageServer {
    backend: JackGraphBackend,
    graph_domain: String,
    documents: BTreeMap<String, DocumentState>,
}

#[derive(Clone, Debug)]
struct DocumentState {
    version: i64,
    text: String,
}

impl Default for JackLanguageServer {
    fn default() -> Self {
        Self::new()
    }
}

impl JackLanguageServer {
    pub fn new() -> Self {
        let graph = Graph::from_fixture(example_graph_fixture()).expect("jack lsp default fixture");
        Self { backend: JackGraphBackend::Trinity(OwnedTrinityQueryableGraph(graph)), graph_domain: "trinity".into(), documents: BTreeMap::new() }
    }

    pub fn set_graph_domain(&mut self, domain: &str) {
        self.graph_domain = domain.to_string();
    }

    pub fn graph_domain(&self) -> &str {
        &self.graph_domain
    }

    pub fn load_fixture_json(&mut self, json: &str) -> Result<(), TrinityJackLspError> {
        self.load_fixture_for_domain(json, &self.graph_domain.clone())
    }

    pub fn load_fixture_for_domain(&mut self, json: &str, domain: &str) -> Result<(), TrinityJackLspError> {
        self.graph_domain = domain.to_string();
        self.backend = match domain {
            "trinity" | "nakagin" => JackGraphBackend::Trinity(OwnedTrinityQueryableGraph(Graph::load_json(json)?)),
            "dag" => JackGraphBackend::Board(BoardQueryableGraph::from_dag_fixture_json(json)?),
            "puzzle2d" | "2d" => JackGraphBackend::Board(BoardQueryableGraph::from_puzzle2d_fixture_json(json)?),
            "puzzle3d" | "3d" => JackGraphBackend::Board(BoardQueryableGraph::from_puzzle3d_fixture_json(json)?),
            "s-workflow" | "s" => JackGraphBackend::Board(BoardQueryableGraph::from_fixture_json(json, Some("s-resources"))?),
            "flow" => JackGraphBackend::Board(BoardQueryableGraph::from_fixture_json(json, Some("flow-dag"))?),
            "sequence" => JackGraphBackend::Board(BoardQueryableGraph::from_fixture_json(json, Some("flow-dag"))?),
            "wires" => JackGraphBackend::Board(BoardQueryableGraph::from_fixture_json(json, Some("wires-default"))?),
            _ => JackGraphBackend::Board(BoardQueryableGraph::from_fixture_json(json, None)?),
        };
        self.refresh_all();
        Ok(())
    }

    pub fn handle_message_json(&mut self, raw: &str) -> String {
        let message: JsonRpcMessage = match serde_json::from_str(raw) {
            Ok(row) => row,
            Err(err) => {
                return serde_json::to_string(&vec![error_response(None, -32700, err.to_string())]).unwrap_or_else(|_| "[]".into());
            }
        };
        let replies = self.handle_message(message);
        serde_json::to_string(&replies).unwrap_or_else(|_| "[]".into())
    }

    pub fn handle_message(&mut self, message: JsonRpcMessage) -> Vec<Value> {
        if message.method.is_none() {
            return Vec::new();
        }
        let method = message.method.unwrap_or_default();
        let id = message.id.clone();
        if method == "initialize" {
            return vec![ok_response(
                id,
                json!({
                    "capabilities": {
                        "textDocumentSync": 1,
                        "completionProvider": { "triggerCharacters": [".", ":"] },
                        "hoverProvider": true,
                        "documentFormattingProvider": true,
                        "semanticTokensProvider": { "legend": { "tokenTypes": ["keyword","ident","number","string","operator","punctuation","error"], "tokenModifiers": [] } }
                    },
                    "serverInfo": { "name": "jack-lsp", "version": "1" }
                }),
            )];
        }
        if method == "initialized" {
            return Vec::new();
        }
        if method == "textDocument/didOpen" {
            let params: DidOpenParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            self.documents.insert(params.text_document.uri.clone(), DocumentState { version: params.text_document.version, text: params.text_document.text });
            return self.publish_for_uri(&params.text_document.uri, id);
        }
        if method == "textDocument/didChange" {
            let params: DidChangeParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let text = params.content_changes.last().map(|c| c.text.clone()).unwrap_or_default();
            if let Some(doc) = self.documents.get_mut(&params.text_document.uri) {
                doc.version = params.text_document.version;
                doc.text = text;
            }
            return self.publish_for_uri(&params.text_document.uri, id);
        }
        if method == "textDocument/completion" {
            let params: TextDocumentPositionParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let Some(doc) = self.documents.get(&params.text_document.uri) else {
                return vec![ok_response(id, json!({ "isIncomplete": false, "items": [] }))];
            };
            let offset = position_to_offset(&doc.text, &params.position);
            let items = complete(self.backend.as_queryable(), &doc.text, offset).into_iter().map(completion_to_lsp).collect::<Vec<_>>();
            return vec![ok_response(id, json!({ "isIncomplete": false, "items": items }))];
        }
        if method == "textDocument/hover" {
            let params: TextDocumentPositionParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let Some(doc) = self.documents.get(&params.text_document.uri) else {
                return vec![ok_response(id, Value::Null)];
            };
            let offset = position_to_offset(&doc.text, &params.position);
            let result = hover(self.backend.as_queryable(), &doc.text, offset).map(hover_to_lsp);
            return vec![ok_response(id, result.unwrap_or(Value::Null))];
        }
        if method == "textDocument/formatting" {
            let params: FormattingParams = match serde_json::from_value(message.params.unwrap_or(Value::Null)) {
                Ok(row) => row,
                Err(err) => return vec![error_response(id, -32602, err.to_string())],
            };
            let Some(doc) = self.documents.get(&params.text_document.uri) else {
                return vec![ok_response(id, json!([]))];
            };
            let formatted = match format(&doc.text) {
                Ok(text) => vec![json!({
                    "range": {
                        "start": { "line": 0, "character": 0 },
                        "end": offset_to_position(&doc.text, doc.text.len())
                    },
                    "newText": text
                })],
                Err(err) => return vec![error_response(id, -32603, err.to_string())],
            };
            return vec![ok_response(id, Value::Array(formatted))];
        }
        if method == "textDocument/semanticTokens/full" {
            let params: TextDocumentRef = match message.params.and_then(|p| serde_json::from_value(p).ok()) {
                Some(row) => row,
                None => return vec![error_response(id, -32602, "missing params".into())],
            };
            let Some(doc) = self.documents.get(&params.uri) else {
                return vec![ok_response(id, json!({ "data": [] }))];
            };
            let tokens = semantic_tokens(&doc.text);
            return vec![ok_response(id, json!({ "tokens": tokens }))];
        }
        vec![error_response(id, -32601, format!("method not found: {method}"))]
    }

    fn refresh_all(&mut self) {
        let uris = self.documents.keys().cloned().collect::<Vec<_>>();
        for uri in uris {
            let _ = self.publish_for_uri(&uri, None);
        }
    }

    fn publish_for_uri(&self, uri: &str, request_id: Option<Value>) -> Vec<Value> {
        let Some(doc) = self.documents.get(uri) else {
            return if let Some(id) = request_id { vec![ok_response(Some(id), Value::Null)] } else { Vec::new() };
        };
        let diagnostics = lint(self.backend.as_queryable(), &doc.text).into_iter().map(|d| diagnostic_to_lsp(&doc.text, d)).collect::<Vec<_>>();
        let tokens = semantic_tokens(&doc.text);
        let mut out = Vec::new();
        if let Some(id) = request_id {
            out.push(ok_response(Some(id), Value::Null));
        }
        out.push(notification("textDocument/publishDiagnostics", json!({ "uri": uri, "diagnostics": diagnostics })));
        out.push(notification("writer/semanticTokens", json!({ "uri": uri, "tokens": tokens })));
        out
    }
}
// #endregion 🔖️JackLanguageServer

// #region 🔖️Wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct JackLspSession {
    server: JackLanguageServer,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl JackLspSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { server: JackLanguageServer::new() }
    }

    #[wasm_bindgen(js_name = loadFixtureJson)]
    pub fn load_fixture_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.server.load_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = loadFixtureForDomain)]
    pub fn load_fixture_for_domain(&mut self, json: &str, graph_domain: &str) -> Result<(), JsValue> {
        self.server.load_fixture_for_domain(json, graph_domain).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setGraphDomain)]
    pub fn set_graph_domain(&mut self, graph_domain: &str) {
        self.server.set_graph_domain(graph_domain);
    }

    #[wasm_bindgen(js_name = handleMessageJson)]
    pub fn handle_message_json(&mut self, raw: &str) -> String {
        self.server.handle_message_json(raw)
    }
}
// #endregion 🔖️Wasm

// #region 🔖️Helpers
fn ok_response(id: Option<Value>, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error_response(id: Option<Value>, code: i64, message: String) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn notification(method: &str, params: Value) -> Value {
    json!({ "jsonrpc": "2.0", "method": method, "params": params })
}

fn offset_to_position(text: &str, offset: usize) -> Value {
    let pos = position_to_offset_struct(text, offset);
    json!({ "line": pos.line, "character": pos.character })
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    line: u32,
    character: u32,
}

fn position_to_offset(text: &str, position: &Position) -> usize {
    let mut offset = 0usize;
    for (i, line) in text.split('\n').enumerate() {
        if i as u32 == position.line {
            return offset + (position.character as usize).min(line.len());
        }
        offset += line.len() + 1;
    }
    text.len()
}

fn position_to_offset_struct(text: &str, offset: usize) -> Pos {
    let clamped = offset.min(text.len());
    let mut line = 0u32;
    let mut last = 0usize;
    for (i, ch) in text.char_indices() {
        if i >= clamped {
            break;
        }
        if ch == '\n' {
            line += 1;
            last = i + 1;
        }
    }
    Pos { line, character: (clamped.saturating_sub(last)) as u32 }
}

fn diagnostic_to_lsp(text: &str, diag: Diagnostic) -> Value {
    json!({
        "range": {
            "start": offset_to_position(text, diag.start),
            "end": offset_to_position(text, diag.end)
        },
        "severity": match diag.severity {
            DiagnosticSeverity::Error => 1,
            DiagnosticSeverity::Warning => 2,
            DiagnosticSeverity::Information => 3,
            DiagnosticSeverity::Hint => 4,
        },
        "code": diag.code,
        "message": diag.message
    })
}

fn completion_to_lsp(item: Completion) -> Value {
    json!({
        "label": item.label,
        "kind": 1,
        "detail": item.detail,
        "insertText": item.insert
    })
}

fn hover_to_lsp(item: Hover) -> Value {
    json!({
        "contents": item.contents,
        "range": {
            "start": { "line": 0, "character": item.start },
            "end": { "line": 0, "character": item.end }
        }
    })
}
// #endregion 🔖️Helpers

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_returns_capabilities() {
        let mut server = JackLanguageServer::new();
        let raw = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(raw)).unwrap();
        assert_eq!(replies[0]["result"]["serverInfo"]["name"], "jack-lsp");
    }

    #[test]
    fn did_open_publishes_diagnostics() {
        let mut server = JackLanguageServer::new();
        let open = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "writer://jack",
                    "languageId": "jack",
                    "version": 1,
                    "text": "RETURN a.name"
                }
            }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&open.to_string())).unwrap();
        assert!(replies.iter().any(|row| row["method"] == "textDocument/publishDiagnostics"));
    }

    fn open_doc(server: &mut JackLanguageServer, uri: &str, id: i64, text: &str) {
        let open = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "textDocument/didOpen",
            "params": { "textDocument": { "uri": uri, "languageId": "jack", "version": 1, "text": text } }
        });
        server.handle_message_json(&open.to_string());
    }

    #[test]
    fn handle_message_json_returns_parse_error_for_invalid_json() {
        let mut server = JackLanguageServer::new();
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json("not json")).unwrap();
        assert_eq!(replies[0]["error"]["code"], -32700);
    }

    #[test]
    fn handle_message_ignores_message_without_method() {
        let mut server = JackLanguageServer::new();
        let message = JsonRpcMessage { id: None, method: None, params: None };
        assert!(server.handle_message(message).is_empty());
    }

    #[test]
    fn initialized_notification_returns_no_replies() {
        let mut server = JackLanguageServer::new();
        let raw = r#"{"jsonrpc":"2.0","method":"initialized"}"#;
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(raw)).unwrap();
        assert!(replies.is_empty());
    }

    #[test]
    fn unknown_method_returns_method_not_found_error() {
        let mut server = JackLanguageServer::new();
        let raw = r#"{"jsonrpc":"2.0","id":14,"method":"foo/bar"}"#;
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(raw)).unwrap();
        assert_eq!(replies[0]["error"]["code"], -32601);
        assert!(replies[0]["error"]["message"].as_str().unwrap().contains("foo/bar"));
    }

    #[test]
    fn did_change_updates_document_and_republishes() {
        let mut server = JackLanguageServer::new();
        open_doc(&mut server, "writer://change", 1, "RETURN a");
        let change = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "textDocument/didChange",
            "params": {
                "textDocument": { "uri": "writer://change", "version": 2 },
                "contentChanges": [{ "text": "RETURN a.name" }]
            }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&change.to_string())).unwrap();
        assert_eq!(replies[0]["result"], Value::Null);
        assert_eq!(replies[0]["id"], 3);
        assert!(replies.iter().any(|row| row["method"] == "textDocument/publishDiagnostics"));
        assert!(replies.iter().any(|row| row["method"] == "writer/semanticTokens"));
    }

    #[test]
    fn did_change_without_prior_open_returns_null_result() {
        let mut server = JackLanguageServer::new();
        let change = json!({
            "jsonrpc": "2.0",
            "id": 99,
            "method": "textDocument/didChange",
            "params": {
                "textDocument": { "uri": "writer://never-opened", "version": 2 },
                "contentChanges": [{ "text": "RETURN a" }]
            }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&change.to_string())).unwrap();
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0]["result"], Value::Null);
        assert_eq!(replies[0]["id"], 99);
    }

    #[test]
    fn completion_for_unknown_document_returns_empty_items() {
        let mut server = JackLanguageServer::new();
        let raw = json!({
            "jsonrpc": "2.0", "id": 4, "method": "textDocument/completion",
            "params": { "textDocument": { "uri": "writer://none" }, "position": { "line": 0, "character": 0 } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert_eq!(replies[0]["result"]["items"], json!([]));
        assert_eq!(replies[0]["result"]["isIncomplete"], false);
    }

    #[test]
    fn completion_for_known_document_returns_matching_kind() {
        let mut server = JackLanguageServer::new();
        let text = "MATCH (a:P";
        open_doc(&mut server, "writer://completion", 1, text);
        let raw = json!({
            "jsonrpc": "2.0", "id": 5, "method": "textDocument/completion",
            "params": { "textDocument": { "uri": "writer://completion" }, "position": { "line": 0, "character": text.len() } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        let items = replies[0]["result"]["items"].as_array().unwrap();
        assert!(items.iter().any(|item| item["label"] == "Piece"));
    }

    #[test]
    fn hover_for_unknown_document_returns_null() {
        let mut server = JackLanguageServer::new();
        let raw = json!({
            "jsonrpc": "2.0", "id": 6, "method": "textDocument/hover",
            "params": { "textDocument": { "uri": "writer://none" }, "position": { "line": 0, "character": 0 } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert_eq!(replies[0]["result"], Value::Null);
    }

    #[test]
    fn hover_for_known_document_returns_keyword_info() {
        let mut server = JackLanguageServer::new();
        open_doc(&mut server, "writer://hover-kw", 1, "MATCH");
        let raw = json!({
            "jsonrpc": "2.0", "id": 7, "method": "textDocument/hover",
            "params": { "textDocument": { "uri": "writer://hover-kw" }, "position": { "line": 0, "character": 2 } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert_eq!(replies[0]["result"]["contents"], "Jack keyword `MATCH`");
    }

    #[test]
    fn hover_for_known_document_returns_null_when_no_match() {
        let mut server = JackLanguageServer::new();
        open_doc(&mut server, "writer://hover-none", 1, "xyz123");
        let raw = json!({
            "jsonrpc": "2.0", "id": 8, "method": "textDocument/hover",
            "params": { "textDocument": { "uri": "writer://hover-none" }, "position": { "line": 0, "character": 2 } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert_eq!(replies[0]["result"], Value::Null);
    }

    #[test]
    fn formatting_for_unknown_document_returns_empty_array() {
        let mut server = JackLanguageServer::new();
        let raw = json!({
            "jsonrpc": "2.0", "id": 9, "method": "textDocument/formatting",
            "params": { "textDocument": { "uri": "writer://none" } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert_eq!(replies[0]["result"], json!([]));
    }

    #[test]
    fn formatting_for_known_document_returns_edit() {
        let mut server = JackLanguageServer::new();
        open_doc(&mut server, "writer://format", 1, "MATCH(a:Piece)RETURN a.name");
        let raw = json!({
            "jsonrpc": "2.0", "id": 10, "method": "textDocument/formatting",
            "params": { "textDocument": { "uri": "writer://format" } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        let edits = replies[0]["result"].as_array().unwrap();
        assert_eq!(edits.len(), 1);
        assert!(edits[0]["newText"].as_str().unwrap().starts_with("MATCH"));
    }

    #[test]
    fn formatting_error_returns_error_response() {
        let mut server = JackLanguageServer::new();
        open_doc(&mut server, "writer://format-err", 1, "MATCH (a:x) WHERE a.p = 'oops");
        let raw = json!({
            "jsonrpc": "2.0", "id": 11, "method": "textDocument/formatting",
            "params": { "textDocument": { "uri": "writer://format-err" } }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert_eq!(replies[0]["error"]["code"], -32603);
    }

    #[test]
    fn semantic_tokens_missing_params_returns_error() {
        let mut server = JackLanguageServer::new();
        let raw = r#"{"jsonrpc":"2.0","id":12,"method":"textDocument/semanticTokens/full"}"#;
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(raw)).unwrap();
        assert_eq!(replies[0]["error"]["code"], -32602);
    }

    #[test]
    fn semantic_tokens_for_unknown_document_returns_empty_data() {
        let mut server = JackLanguageServer::new();
        let raw = json!({
            "jsonrpc": "2.0", "id": 13, "method": "textDocument/semanticTokens/full",
            "params": { "uri": "writer://none" }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert_eq!(replies[0]["result"], json!({ "data": [] }));
    }

    #[test]
    fn semantic_tokens_for_known_document_returns_tokens() {
        let mut server = JackLanguageServer::new();
        open_doc(&mut server, "writer://tokens", 1, "RETURN a");
        let raw = json!({
            "jsonrpc": "2.0", "id": 15, "method": "textDocument/semanticTokens/full",
            "params": { "uri": "writer://tokens" }
        });
        let replies: Vec<Value> = serde_json::from_str(&server.handle_message_json(&raw.to_string())).unwrap();
        assert!(replies[0]["result"]["tokens"].is_array());
        assert!(!replies[0]["result"]["tokens"].as_array().unwrap().is_empty());
    }

    #[test]
    fn load_fixture_for_domain_trinity_and_nakagin_succeed() {
        let mut server = JackLanguageServer::new();
        let fixture_json = trinity::core::example_graph_fixture_json();
        server.load_fixture_for_domain(&fixture_json, "trinity").unwrap();
        assert_eq!(server.graph_domain(), "trinity");
        server.load_fixture_for_domain(&fixture_json, "nakagin").unwrap();
        assert_eq!(server.graph_domain(), "nakagin");
    }

    #[test]
    fn load_fixture_for_domain_board_aliases_succeed() {
        let mut server = JackLanguageServer::new();
        for domain in ["dag", "puzzle2d", "2d", "s-workflow", "s", "flow", "sequence", "wires", "some-unlisted-domain"] {
            server.load_fixture_for_domain("{}", domain).unwrap();
            assert_eq!(server.graph_domain(), domain);
        }
    }

    #[test]
    fn load_fixture_for_domain_puzzle3d_converts_objects_array() {
        let mut server = JackLanguageServer::new();
        let fixture = r#"{"objects":[{"id":"o1","kind":"Widget","name":"Thing"}]}"#;
        server.load_fixture_for_domain(fixture, "puzzle3d").unwrap();
        server.load_fixture_for_domain(fixture, "3d").unwrap();
    }

    #[test]
    fn load_fixture_for_domain_invalid_json_returns_graph_error_for_trinity() {
        let mut server = JackLanguageServer::new();
        let err = server.load_fixture_for_domain("not json", "trinity").unwrap_err();
        assert!(matches!(err, TrinityJackLspError::Graph(_)));
    }

    #[test]
    fn load_fixture_for_domain_invalid_json_returns_dsl_error_for_board() {
        let mut server = JackLanguageServer::new();
        let err = server.load_fixture_for_domain("not json", "dag").unwrap_err();
        assert!(matches!(err, TrinityJackLspError::Dsl(_)));
    }

    #[test]
    fn load_fixture_json_uses_current_domain() {
        let mut server = JackLanguageServer::new();
        server.set_graph_domain("dag");
        server.load_fixture_json("{}").unwrap();
        assert_eq!(server.graph_domain(), "dag");
    }

    #[test]
    fn default_matches_new_domain() {
        let server = JackLanguageServer::default();
        assert_eq!(server.graph_domain(), "trinity");
    }

    #[test]
    fn load_fixture_for_domain_refreshes_open_documents() {
        let mut server = JackLanguageServer::new();
        open_doc(&mut server, "writer://refresh", 1, "RETURN a");
        let fixture_json = trinity::core::example_graph_fixture_json();
        assert!(server.load_fixture_for_domain(&fixture_json, "trinity").is_ok());
    }

    #[test]
    fn position_to_offset_clamps_character_to_line_length() {
        let text = "ab\ncdef";
        let pos = Position { line: 0, character: 10 };
        assert_eq!(position_to_offset(text, &pos), 2);
    }

    #[test]
    fn position_to_offset_returns_text_len_when_line_out_of_range() {
        let text = "ab\ncdef";
        let pos = Position { line: 5, character: 0 };
        assert_eq!(position_to_offset(text, &pos), text.len());
    }

    #[test]
    fn offset_to_position_and_position_to_offset_round_trip() {
        let text = "ab\ncdef";
        let pos = offset_to_position(text, 5);
        assert_eq!(pos, json!({ "line": 1, "character": 2 }));
        assert_eq!(position_to_offset(text, &Position { line: 1, character: 2 }), 5);
    }
}
// #endregion 🔖️Tests
