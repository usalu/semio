//! 📡️ LSP 3.17 server over JSON-RPC 2.0 for a [`LanguageEngine`]: documents, diagnostics, completion, hover, formatting and semantic tokens.
//!
//! The method roster, error codes, legends and kind mappings are read from the embedded protocol schema `🔖️protocol/🔣️.json`.

use crate::position::{LineIndex, Position};
use semio_framework_graph_dsl::{Completion, Diagnostic, DiagnosticSeverity, Hover, SemanticToken};
use semio_framework_pack_json::{array, object, JsonMemberPolicy, Object, Value};
use std::collections::BTreeMap;
use std::sync::OnceLock;

const SCHEMA: &str = include_str!("../🔖️protocol/🔣️.json");

//#region 🔖️Engine
/// 🧠️ The language analysis a server drives; offsets are bytes of the document text.
pub trait LanguageEngine {
    fn diagnostics(&self, source: &str) -> Vec<Diagnostic>;
    fn complete(&self, source: &str, offset: usize) -> Vec<Completion>;
    fn hover(&self, source: &str, offset: usize) -> Option<Hover>;
    fn semantic_tokens(&self, source: &str) -> Vec<SemanticToken>;
    fn format(&self, source: &str) -> Result<String, String>;
    fn extension(&mut self, method: &str, params: &Value) -> Option<Result<Value, String>>;
}
//#endregion 🔖️Engine

//#region 🔖️Protocol
struct Protocol {
    name: String,
    version: String,
    maximum_document_bytes: usize,
    capabilities: Value,
    error_codes: BTreeMap<String, i64>,
    severities: BTreeMap<String, i64>,
    completion_kinds: BTreeMap<String, i64>,
    token_types: Vec<String>,
    token_classes: BTreeMap<String, i64>,
    source: String,
    suppressed_prefix: String,
    too_large: String,
    requests: Vec<String>,
    notifications: Vec<String>,
}

fn numbers(value: Option<&Value>) -> BTreeMap<String, i64> {
    value.and_then(Value::as_object).map(|members| members.iter().filter_map(|(key, member)| member.as_i64().map(|number| (key.to_string(), number))).collect()).unwrap_or_default()
}

fn texts(value: Option<&Value>) -> Vec<String> {
    value.and_then(Value::as_array).map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn text(value: Option<&Value>) -> String {
    value.and_then(Value::as_str).unwrap_or_default().to_string()
}

fn protocol() -> &'static Protocol {
    static PROTOCOL: OnceLock<Protocol> = OnceLock::new();
    PROTOCOL.get_or_init(|| {
        let schema = semio_framework_pack_json::parse(SCHEMA, JsonMemberPolicy::Reject).expect("the embedded Jack protocol schema is valid JSON");
        let server = schema.get("server");
        let mut capabilities = schema.get("capabilities").cloned().unwrap_or(Value::Null);
        let token_types = texts(schema.get("semanticTokenTypes"));
        if let Some(provider) = capabilities.get_mut("semanticTokensProvider").and_then(Value::as_object_mut) {
            provider.insert("legend", object([("tokenTypes".to_string(), array(token_types.iter().cloned().map(Value::from))), ("tokenModifiers".to_string(), array([]))]));
        }
        if let Some(members) = capabilities.as_object_mut() {
            members.insert("positionEncoding", Value::from(text(server.and_then(|row| row.get("positionEncoding")))));
        }
        Protocol {
            name: text(server.and_then(|row| row.get("name"))),
            version: text(server.and_then(|row| row.get("version"))),
            maximum_document_bytes: server.and_then(|row| row.get("maximumDocumentBytes")).and_then(Value::as_u64).unwrap_or(0) as usize,
            capabilities,
            error_codes: numbers(schema.get("errorCodes")),
            severities: numbers(schema.get("diagnosticSeverities")),
            completion_kinds: numbers(schema.get("completionKinds")),
            token_types,
            token_classes: numbers(schema.get("semanticTokenClasses")),
            source: text(schema.get("diagnosticSource")),
            suppressed_prefix: text(schema.get("graphlessSuppressedCodePrefix")),
            too_large: text(schema.get("documentTooLargeCode")),
            requests: texts(schema.get("requests")),
            notifications: texts(schema.get("notifications")),
        }
    })
}

/// 📜️ The numeric JSON-RPC error code the schema names.
pub fn error_code(name: &str) -> i64 {
    protocol().error_codes[name]
}

/// 📜️ The code a graph-less session never reports: semantic kind checks need the manifest of a loaded snapshot.
pub fn graphless_suppressed_code_prefix() -> &'static str {
    &protocol().suppressed_prefix
}

/// 📜️ Every request method the schema declares.
pub fn declared_requests() -> &'static [String] {
    &protocol().requests
}

/// 📜️ Every notification method the schema declares.
pub fn declared_notifications() -> &'static [String] {
    &protocol().notifications
}
//#endregion 🔖️Protocol

//#region 🔖️Messages
struct Failure {
    code: &'static str,
    message: String,
}

type Outcome = Result<Value, Failure>;

fn failure(code: &'static str, message: impl Into<String>) -> Failure {
    Failure { code, message: message.into() }
}

fn envelope(members: impl IntoIterator<Item = (String, Value)>) -> Value {
    object([("jsonrpc".to_string(), Value::from("2.0"))].into_iter().chain(members))
}

fn response(id: Value, result: Value) -> Value {
    envelope([("id".to_string(), id), ("result".to_string(), result)])
}

fn error_response(id: Value, code: i64, message: &str) -> Value {
    envelope([("id".to_string(), id), ("error".to_string(), object([("code".to_string(), Value::from(code)), ("message".to_string(), Value::from(message))]))])
}

fn notification(method: &str, params: Value) -> Value {
    envelope([("method".to_string(), Value::from(method)), ("params".to_string(), params)])
}

fn position_value(position: Position) -> Value {
    object([("line".to_string(), Value::from(position.line)), ("character".to_string(), Value::from(position.character))])
}

fn range_value(range: crate::position::Range) -> Value {
    object([("start".to_string(), position_value(range.start)), ("end".to_string(), position_value(range.end))])
}

fn read_position(value: &Value) -> Option<Position> {
    Some(Position { line: u32::try_from(value.get("line")?.as_u64()?).ok()?, character: u32::try_from(value.get("character")?.as_u64()?).ok()? })
}
//#endregion 🔖️Messages

//#region 🔖️Server
struct Document {
    text: String,
    version: Option<i64>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Created,
    Running,
    ShutDown,
    Exited,
}

/// 📡️ One language server session: feed it client messages, collect the messages it sends back.
pub struct LanguageServer<E: LanguageEngine> {
    engine: E,
    documents: BTreeMap<String, Document>,
    phase: Phase,
}

impl<E: LanguageEngine> LanguageServer<E> {
    pub fn new(engine: E) -> Self {
        Self { engine, documents: BTreeMap::new(), phase: Phase::Created }
    }

    pub fn engine(&self) -> &E {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut E {
        &mut self.engine
    }

    pub fn has_exited(&self) -> bool {
        self.phase == Phase::Exited
    }

    pub fn handle(&mut self, message: &Value) -> Vec<Value> {
        let protocol = protocol();
        let Some(members) = message.as_object() else {
            return vec![error_response(Value::Null, protocol.error_codes["invalidRequest"], "a JSON-RPC message is an object")];
        };
        let id = members.get("id").cloned();
        let Some(method) = members.get("method").and_then(Value::as_str) else {
            if members.contains_key("result") || members.contains_key("error") {
                return Vec::new();
            }
            return id.map(|id| vec![error_response(id, protocol.error_codes["invalidRequest"], "a JSON-RPC request names its method")]).unwrap_or_default();
        };
        let params = members.get("params").cloned().unwrap_or(Value::Null);
        match id {
            Some(id) => self.request(id, method, &params),
            None => self.notify(method, &params),
        }
    }

    fn request(&mut self, id: Value, method: &str, params: &Value) -> Vec<Value> {
        let protocol = protocol();
        let code = |name: &str| protocol.error_codes[name];
        match (self.phase, method) {
            (Phase::Exited, _) => return Vec::new(),
            (Phase::Created, "initialize") => {
                self.phase = Phase::Running;
                let result = object([
                    ("capabilities".to_string(), protocol.capabilities.clone()),
                    ("serverInfo".to_string(), object([("name".to_string(), Value::from(protocol.name.as_str())), ("version".to_string(), Value::from(protocol.version.as_str()))])),
                ]);
                return vec![response(id, result)];
            }
            (Phase::Created, _) => return vec![error_response(id, code("serverNotInitialized"), "the server is not initialized")],
            (_, "initialize") => return vec![error_response(id, code("invalidRequest"), "the server is already initialized")],
            (Phase::ShutDown, _) => return vec![error_response(id, code("invalidRequest"), "the server is shut down")],
            (Phase::Running, "shutdown") => {
                self.phase = Phase::ShutDown;
                return vec![response(id, Value::Null)];
            }
            _ => {}
        }
        let outcome = match method {
            "textDocument/completion" => self.completion(params),
            "textDocument/hover" => self.hover(params),
            "textDocument/formatting" => self.formatting(params),
            "textDocument/semanticTokens/full" => self.semantic_tokens(params),
            _ => match self.engine.extension(method, params) {
                Some(Ok(result)) => {
                    let mut out = vec![response(id, result)];
                    out.extend(self.publish_all());
                    return out;
                }
                Some(Err(message)) => Err(failure("requestFailed", message)),
                None => Err(failure("methodNotFound", format!("unknown method {method}"))),
            },
        };
        vec![match outcome {
            Ok(result) => response(id, result),
            Err(error) => error_response(id, code(error.code), &error.message),
        }]
    }

    fn notify(&mut self, method: &str, params: &Value) -> Vec<Value> {
        match (self.phase, method) {
            (Phase::Exited, _) => Vec::new(),
            (_, "exit") => {
                self.phase = Phase::Exited;
                Vec::new()
            }
            (Phase::Running, "textDocument/didOpen") => self.did_open(params),
            (Phase::Running, "textDocument/didChange") => self.did_change(params),
            (Phase::Running, "textDocument/didClose") => self.did_close(params),
            _ => Vec::new(),
        }
    }

    fn document(&self, params: &Value) -> Result<&Document, Failure> {
        let uri = params.get("textDocument").and_then(|row| row.get("uri")).and_then(Value::as_str).ok_or_else(|| failure("invalidParams", "textDocument.uri is required"))?;
        self.documents.get(uri).ok_or_else(|| failure("invalidParams", format!("document {uri} is not open")))
    }

    fn analyzable(document: &Document) -> bool {
        document.text.len() <= protocol().maximum_document_bytes
    }

    fn cursor(&self, params: &Value) -> Result<(&Document, usize), Failure> {
        let document = self.document(params)?;
        let position = params.get("position").and_then(read_position).ok_or_else(|| failure("invalidParams", "position is required"))?;
        Ok((document, LineIndex::new(&document.text).offset(position)))
    }

    fn completion(&self, params: &Value) -> Outcome {
        let (document, offset) = self.cursor(params)?;
        let protocol = protocol();
        let items = if Self::analyzable(document) { self.engine.complete(&document.text, offset) } else { Vec::new() };
        let items = items.into_iter().map(|item| {
            let kind = protocol.completion_kinds.get(&item.kind).or_else(|| protocol.completion_kinds.get("default")).copied().unwrap_or(1);
            let mut members = vec![("label".to_string(), Value::from(item.label)), ("kind".to_string(), Value::from(kind)), ("insertText".to_string(), Value::from(item.insert))];
            if let Some(detail) = item.detail {
                members.push(("detail".to_string(), Value::from(detail)));
            }
            object(members)
        });
        Ok(object([("isIncomplete".to_string(), Value::from(false)), ("items".to_string(), array(items))]))
    }

    fn hover(&self, params: &Value) -> Outcome {
        let (document, offset) = self.cursor(params)?;
        if !Self::analyzable(document) {
            return Ok(Value::Null);
        }
        let index = LineIndex::new(&document.text);
        Ok(self.engine.hover(&document.text, offset).map_or(Value::Null, |hover| {
            object([
                ("contents".to_string(), object([("kind".to_string(), Value::from("markdown")), ("value".to_string(), Value::from(hover.contents))])),
                ("range".to_string(), range_value(index.range(hover.start, hover.end))),
            ])
        }))
    }

    fn formatting(&self, params: &Value) -> Outcome {
        let document = self.document(params)?;
        if !Self::analyzable(document) {
            return Err(failure("requestFailed", "the document exceeds the analysis limit"));
        }
        let formatted = self.engine.format(&document.text).map_err(|message| failure("requestFailed", message))?;
        if formatted == document.text {
            return Ok(array([]));
        }
        let range = LineIndex::new(&document.text).whole();
        Ok(array([object([("range".to_string(), range_value(range)), ("newText".to_string(), Value::from(formatted))])]))
    }

    fn semantic_tokens(&self, params: &Value) -> Outcome {
        let document = self.document(params)?;
        let tokens = if Self::analyzable(document) { self.engine.semantic_tokens(&document.text) } else { Vec::new() };
        Ok(object([("data".to_string(), array(encode_semantic_tokens(&document.text, tokens).into_iter().map(Value::from)))]))
    }

    fn did_open(&mut self, params: &Value) -> Vec<Value> {
        let Some(row) = params.get("textDocument") else { return Vec::new() };
        let (Some(uri), Some(text)) = (row.get("uri").and_then(Value::as_str), row.get("text").and_then(Value::as_str)) else { return Vec::new() };
        self.documents.insert(uri.to_string(), Document { text: text.to_string(), version: row.get("version").and_then(Value::as_i64) });
        self.publish(uri)
    }

    fn did_change(&mut self, params: &Value) -> Vec<Value> {
        let Some(row) = params.get("textDocument") else { return Vec::new() };
        let Some(uri) = row.get("uri").and_then(Value::as_str) else { return Vec::new() };
        let Some(changes) = params.get("contentChanges").and_then(Value::as_array) else { return Vec::new() };
        let Some(document) = self.documents.get_mut(uri) else { return Vec::new() };
        let mut text = document.text.clone();
        for change in changes {
            if apply_change(&mut text, change).is_none() {
                return Vec::new();
            }
        }
        document.text = text;
        document.version = row.get("version").and_then(Value::as_i64).or(document.version);
        self.publish(uri)
    }

    fn did_close(&mut self, params: &Value) -> Vec<Value> {
        let Some(uri) = params.get("textDocument").and_then(|row| row.get("uri")).and_then(Value::as_str) else { return Vec::new() };
        if self.documents.remove(uri).is_none() {
            return Vec::new();
        }
        vec![notification("textDocument/publishDiagnostics", object([("uri".to_string(), Value::from(uri)), ("diagnostics".to_string(), array([]))]))]
    }

    fn publish_all(&self) -> Vec<Value> {
        self.documents.keys().flat_map(|uri| self.publish(uri)).collect()
    }

    fn publish(&self, uri: &str) -> Vec<Value> {
        let Some(document) = self.documents.get(uri) else { return Vec::new() };
        let protocol = protocol();
        let diagnostics = if Self::analyzable(document) {
            self.engine.diagnostics(&document.text)
        } else {
            vec![Diagnostic { start: 0, end: 0, severity: DiagnosticSeverity::Error, message: "the document exceeds the analysis limit".to_string(), code: Some(protocol.too_large.clone()) }]
        };
        let index = LineIndex::new(&document.text);
        let rows = diagnostics.into_iter().map(|diagnostic| {
            let severity = match diagnostic.severity {
                DiagnosticSeverity::Error => "error",
                DiagnosticSeverity::Warning => "warning",
                DiagnosticSeverity::Information => "information",
                DiagnosticSeverity::Info => "info",
            };
            let mut members = vec![
                ("range".to_string(), range_value(index.range(diagnostic.start, diagnostic.end))),
                ("severity".to_string(), Value::from(protocol.severities.get(severity).copied().unwrap_or(1))),
                ("source".to_string(), Value::from(protocol.source.as_str())),
                ("message".to_string(), Value::from(diagnostic.message)),
            ];
            if let Some(code) = diagnostic.code {
                members.push(("code".to_string(), Value::from(code)));
            }
            object(members)
        });
        let mut params = Object::new();
        params.insert("uri", Value::from(uri));
        if let Some(version) = document.version {
            params.insert("version", Value::from(version));
        }
        params.insert("diagnostics", array(rows));
        vec![notification("textDocument/publishDiagnostics", Value::Object(params))]
    }
}
//#endregion 🔖️Server

//#region 🔖️Text
fn apply_change(text: &mut String, change: &Value) -> Option<()> {
    let replacement = change.get("text")?.as_str()?;
    match change.get("range") {
        None | Some(Value::Null) => *text = replacement.to_string(),
        Some(range) => {
            let index = LineIndex::new(text);
            let start = index.offset(read_position(range.get("start")?)?);
            let end = index.offset(read_position(range.get("end")?)?).max(start);
            text.replace_range(start..end, replacement);
        }
    }
    Some(())
}

fn encode_semantic_tokens(text: &str, mut tokens: Vec<SemanticToken>) -> Vec<u32> {
    let protocol = protocol();
    tokens.sort_by_key(|token| (token.start, token.end));
    let index = LineIndex::new(text);
    let mut data = Vec::new();
    let mut previous = Position::default();
    let mut previous_end = Position::default();
    for token in tokens {
        let Some(type_index) = protocol.token_classes.get(&token.class).and_then(|number| u32::try_from(*number).ok()).filter(|number| (*number as usize) < protocol.token_types.len()) else { continue };
        let start = index.position(token.start);
        let end = index.position(token.end.max(token.start));
        if start < previous_end {
            continue;
        }
        for line in start.line..=end.line {
            let from = if line == start.line { start.character } else { 0 };
            let to = if line == end.line { end.character } else { index.line_length(line) };
            if to <= from {
                continue;
            }
            let here = Position { line, character: from };
            data.push(here.line - previous.line);
            data.push(if here.line == previous.line { here.character - previous.character } else { here.character });
            data.push(to - from);
            data.push(type_index);
            data.push(0);
            previous = here;
        }
        previous_end = end;
    }
    data
}
//#endregion 🔖️Text
