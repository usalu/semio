//! 🧪️ Jack language server laws: positions, the shared protocol vectors (checked against `serde_json` as the independent JSON oracle), the schema roster and the server over a scripted engine.

use semio_framework_graph_dsl::{Completion, Diagnostic, DiagnosticSeverity, Hover, SemanticToken};
use semio_framework_pack_json::Value;
use semio_s_artifact_trinity_jack_lsp::position::{LineIndex, Position};
use semio_s_artifact_trinity_jack_lsp::server::{declared_notifications, declared_requests, error_code, LanguageEngine, LanguageServer};
use semio_s_artifact_trinity_jack_lsp::{JackEngine, JackLanguageSession};
use serde_json::{json, Value as Json};
use std::cell::Cell;

const POSITIONS: &str = include_str!("../📍️position/🔣️.json");
const PROTOCOL: &str = include_str!("../🔖️protocol/🔣️.json");
const SCHEMA: &str = include_str!("../../🔖️protocol/🔣️.json");

fn matches(expected: &Json, actual: &Json) -> bool {
    match expected {
        Json::Object(members) if members.len() == 1 && members.contains_key("$any") => true,
        Json::Object(members) if members.len() == 1 && members.contains_key("$contains") => actual.as_str().is_some_and(|text| members["$contains"].as_str().is_some_and(|needle| text.contains(needle))),
        Json::Object(members) if members.len() == 1 && members.contains_key("$prefix") => actual.as_array().is_some_and(|items| members["$prefix"].as_array().is_some_and(|head| items.len() >= head.len() && head.iter().zip(items).all(|(want, got)| matches(want, got)))),
        Json::Object(members) => actual.as_object().is_some_and(|object| members.iter().all(|(key, want)| object.get(key).is_some_and(|got| matches(want, got)))),
        Json::Array(items) => actual.as_array().is_some_and(|candidates| candidates.len() == items.len() && items.iter().zip(candidates).all(|(want, got)| matches(want, got))),
        scalar => scalar == actual,
    }
}

#[semio_framework_async_macros::async_test]
async fn positions_match_the_shared_vector_and_round_trip() {
    let vector: Json = serde_json::from_str(POSITIONS).unwrap();
    for case in vector["cases"].as_array().unwrap() {
        let text = case["text"].as_str().unwrap();
        let offset = case["offset"].as_u64().unwrap() as usize;
        let want = Position { line: case["line"].as_u64().unwrap() as u32, character: case["character"].as_u64().unwrap() as u32 };
        let index = LineIndex::new(text);
        assert_eq!(index.position(offset), want, "{}", case["name"]);
        if case["oracle"].as_bool().unwrap() {
            assert_eq!(index.offset(want), offset, "round trip of {}", case["name"]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn positions_clamp_lines_and_characters_to_the_document() {
    let index = LineIndex::new("ab\ncd");
    assert_eq!(index.offset(Position { line: 0, character: 99 }), 2);
    assert_eq!(index.offset(Position { line: 1, character: 99 }), 5);
    assert_eq!(index.offset(Position { line: 9, character: 0 }), 5);
    assert_eq!(index.line_count(), 2);
    assert_eq!(index.line_length(0), 2);
    assert_eq!(index.line_length(9), 0);
    assert_eq!(index.whole().end, Position { line: 1, character: 2 });
}

#[semio_framework_async_macros::async_test]
async fn the_session_conforms_to_every_shared_protocol_vector() {
    let vector: Json = serde_json::from_str(PROTOCOL).unwrap();
    for session in vector["sessions"].as_array().unwrap() {
        let mut jack = JackLanguageSession::new();
        for (number, step) in session["steps"].as_array().unwrap().iter().enumerate() {
            let message = if let Some(raw) = step.get("raw") {
                raw.as_str().unwrap().to_string()
            } else if let Some(name) = step.get("loadSnapshot") {
                json!({ "jsonrpc": "2.0", "id": step["id"], "method": "jack/loadSnapshot", "params": { "json": vector["snapshots"][name.as_str().unwrap()].to_string() } }).to_string()
            } else {
                step["send"].to_string()
            };
            let reply: Json = serde_json::from_str(&jack.handle_message_json(&message)).unwrap();
            assert!(matches(&step["expect"], &reply), "session {} step {number}: expected {} but the server answered {reply}", session["name"], step["expect"]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn every_declared_request_is_known_to_the_initialized_server() {
    let mut jack = JackLanguageSession::new();
    jack.handle_message_json(r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{}}"#);
    for method in declared_requests().iter().filter(|method| !matches!(method.as_str(), "initialize" | "shutdown")) {
        let reply: Json = serde_json::from_str(&jack.handle_message_json(&json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": {} }).to_string())).unwrap();
        assert_ne!(reply[0]["error"]["code"], json!(error_code("methodNotFound")), "{method}");
    }
    assert!(declared_notifications().iter().all(|method| !method.is_empty()));
    let schema: Json = serde_json::from_str(SCHEMA).unwrap();
    assert_eq!(schema["requests"].as_array().unwrap().len(), declared_requests().len());
    assert_eq!(schema["notifications"].as_array().unwrap().len(), declared_notifications().len());
}

#[semio_framework_async_macros::async_test]
async fn a_snapshot_with_an_invalid_manifest_is_refused_and_leaves_the_session_graphless() {
    let mut engine = JackEngine::default();
    assert!(!engine.has_snapshot());
    for broken in [r#"{"manifest":5,"nodes":[],"edges":[]}"#, r#"{"manifest":{"nodeKinds":[{"nope":1}]}}"#, r#"{"manifest":{"edgeKinds":"Connection"}}"#] {
        let error = engine.load_snapshot(broken).unwrap_err();
        assert!(error.contains("manifest"), "{error}");
        assert!(!engine.has_snapshot());
    }
    engine.load_snapshot(r#"{"nodes":[],"edges":[]}"#).unwrap();
    assert!(engine.has_snapshot());
}

#[semio_framework_async_macros::async_test]
async fn a_manifest_that_disagrees_with_the_snapshot_identity_is_refused() {
    let mut engine = JackEngine::default();
    let error = engine.load_snapshot(r#"{"manifestId":"a","manifest":{"id":"b","nodeKinds":[]},"nodes":[],"edges":[]}"#).unwrap_err();
    assert!(!error.is_empty());
    assert!(!engine.has_snapshot());
}

#[semio_framework_async_macros::async_test]
async fn both_manifest_row_shapes_declare_the_same_kinds() {
    let mut jack = JackLanguageSession::new();
    jack.handle_message_json(r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{}}"#);
    for manifest in [r#"{"nodeKinds":[{"name":"Beam"}]}"#, r#"{"schema":"s","id":"m","nodeKinds":[{"id":"Beam"}]}"#] {
        jack.load_snapshot(&format!(r#"{{"manifest":{manifest},"nodes":[],"edges":[]}}"#)).unwrap();
        jack.handle_message_json(r#"{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"u","version":1,"text":"MATCH (a:Beam) RETURN a"}}}"#);
        let reply: Json = serde_json::from_str(&jack.handle_message_json(r#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":"u","version":2},"contentChanges":[{"text":"MATCH (a:Beam) RETURN a"}]}}"#)).unwrap();
        assert_eq!(reply[0]["params"]["diagnostics"], json!([]), "{manifest}");
    }
}

struct ScriptedEngine {
    calls: Cell<usize>,
}

impl LanguageEngine for ScriptedEngine {
    fn diagnostics(&self, source: &str) -> Vec<Diagnostic> {
        self.calls.set(self.calls.get() + 1);
        vec![Diagnostic { start: 0, end: source.len(), severity: DiagnosticSeverity::Warning, message: source.to_string(), code: None }]
    }

    fn complete(&self, _source: &str, _offset: usize) -> Vec<Completion> {
        Vec::new()
    }

    fn hover(&self, _source: &str, _offset: usize) -> Option<Hover> {
        None
    }

    fn semantic_tokens(&self, _source: &str) -> Vec<SemanticToken> {
        let token = |start, end, class: &str| SemanticToken { start, end, class: class.to_string() };
        vec![token(14, 15, "ident"), token(0, 5, "keyword"), token(6, 13, "string"), token(0, 3, "keyword"), token(5, 6, "unknown")]
    }

    fn format(&self, _source: &str) -> Result<String, String> {
        Err("no formatter".to_string())
    }

    fn extension(&mut self, _method: &str, _params: &Value) -> Option<Result<Value, String>> {
        None
    }
}

fn scripted() -> LanguageServer<ScriptedEngine> {
    let mut server = LanguageServer::new(ScriptedEngine { calls: Cell::new(0) });
    server.handle(&semio_framework_pack_json::parse(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    server
}

fn send(server: &mut LanguageServer<ScriptedEngine>, message: Json) -> Json {
    let value = semio_framework_pack_json::parse(&message.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let replies = server.handle(&value);
    serde_json::from_str(&semio_framework_pack_json::to_string(&semio_framework_pack_json::array(replies))).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn semantic_tokens_are_sorted_delta_encoded_split_per_line_and_never_overlap() {
    let mut server = scripted();
    send(&mut server, json!({ "jsonrpc": "2.0", "method": "textDocument/didOpen", "params": { "textDocument": { "uri": "u", "version": 1, "text": "MATCH 'ab\ncd' x" } } }));
    let reply = send(&mut server, json!({ "jsonrpc": "2.0", "id": 2, "method": "textDocument/semanticTokens/full", "params": { "textDocument": { "uri": "u" } } }));
    assert_eq!(reply[0]["result"]["data"], json!([0, 0, 3, 0, 0, 0, 6, 3, 3, 0, 1, 0, 3, 3, 0, 0, 4, 1, 1, 0]));
}

#[semio_framework_async_macros::async_test]
async fn incremental_changes_apply_in_order_over_utf16_ranges() {
    let mut server = scripted();
    send(&mut server, json!({ "jsonrpc": "2.0", "method": "textDocument/didOpen", "params": { "textDocument": { "uri": "u", "version": 1, "text": "a😀b" } } }));
    let reply = send(
        &mut server,
        json!({ "jsonrpc": "2.0", "method": "textDocument/didChange", "params": { "textDocument": { "uri": "u", "version": 2 }, "contentChanges": [
            { "range": { "start": { "line": 0, "character": 3 }, "end": { "line": 0, "character": 3 } }, "text": "X" },
            { "range": { "start": { "line": 0, "character": 1 }, "end": { "line": 0, "character": 3 } }, "text": "-" }
        ] } }),
    );
    assert_eq!(reply[0]["params"]["diagnostics"][0]["message"], "a-Xb");
    assert_eq!(reply[0]["params"]["version"], 2);
    assert_eq!(reply[0]["params"]["diagnostics"][0]["severity"], 2);
    let skipped = send(&mut server, json!({ "jsonrpc": "2.0", "method": "textDocument/didChange", "params": { "textDocument": { "uri": "u", "version": 3 }, "contentChanges": [{ "range": { "start": { "line": 0 }, "end": { "line": 0, "character": 1 } }, "text": "Q" }] } }));
    assert_eq!(skipped, json!([]));
    let kept = send(&mut server, json!({ "jsonrpc": "2.0", "id": 9, "method": "textDocument/formatting", "params": { "textDocument": { "uri": "u" } } }));
    assert_eq!(kept[0]["error"]["code"], json!(error_code("requestFailed")));
}

#[semio_framework_async_macros::async_test]
async fn a_document_over_the_schema_limit_is_not_analyzed() {
    let mut server = scripted();
    let text = "x".repeat(4 * 1024 * 1024 + 1);
    let reply = send(&mut server, json!({ "jsonrpc": "2.0", "method": "textDocument/didOpen", "params": { "textDocument": { "uri": "big", "version": 1, "text": text } } }));
    assert_eq!(reply[0]["params"]["diagnostics"][0]["code"], "jack/document-too-large");
    assert_eq!(server.engine().calls.get(), 0);
    let tokens = send(&mut server, json!({ "jsonrpc": "2.0", "id": 3, "method": "textDocument/semanticTokens/full", "params": { "textDocument": { "uri": "big" } } }));
    assert_eq!(tokens[0]["result"]["data"], json!([]));
}

#[semio_framework_async_macros::async_test]
async fn exit_ends_the_session_and_silences_it() {
    let mut jack = JackLanguageSession::new();
    assert!(!jack.has_exited());
    jack.handle_message_json(r#"{"jsonrpc":"2.0","method":"exit"}"#);
    assert!(jack.has_exited());
    assert_eq!(jack.handle_message_json(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#), "[]");
}

const SHELL_SNAPSHOT: &str = include_str!("../../../../🐚️shell/🧪️tests/🧫️fixtures/🔱️mini-tower/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_shell_example_snapshot_serves_its_nakagin_kinds_and_its_node() {
    let mut jack = JackLanguageSession::new();
    jack.handle_message_json(r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{}}"#);
    jack.load_snapshot(SHELL_SNAPSHOT).unwrap();
    jack.handle_message_json(r#"{"jsonrpc":"2.0","method":"textDocument/didOpen","params":{"textDocument":{"uri":"u","version":1,"text":"MATCH (a:"}}}"#);
    let reply: Json = serde_json::from_str(&jack.handle_message_json(r#"{"jsonrpc":"2.0","id":1,"method":"textDocument/completion","params":{"textDocument":{"uri":"u"},"position":{"line":0,"character":9}}}"#)).unwrap();
    let labels: Vec<&str> = reply[0]["result"]["items"].as_array().unwrap().iter().filter_map(|item| item["label"].as_str()).collect();
    assert!(labels.contains(&"Piece") && labels.contains(&"Balcony"), "{labels:?}");
    let checked: Json = serde_json::from_str(&jack.handle_message_json(r#"{"jsonrpc":"2.0","method":"textDocument/didChange","params":{"textDocument":{"uri":"u","version":2},"contentChanges":[{"text":"MATCH (a:Gizmo) RETURN a"}]}}"#)).unwrap();
    assert_eq!(checked[0]["params"]["diagnostics"][0]["code"], "jack/unknown-node-kind");
}
