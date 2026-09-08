
use super::*;

fn sample_event(input_redacted: serde_json::Value) -> AgentAuditEvent {
    AgentAuditEvent {
        invocation_id: "inv-1".into(),
        ts_ms: 1_000,
        principal: "agent:local".into(),
        session: "sess_1".into(),
        capability: "cad.viewport.translateSelection".into(),
        input_hash: "blake3:abc".into(),
        input_redacted,
        decision: AuditDecision::Allowed,
        preview_hash: None,
        txn_id: None,
        edit_ids: vec![],
        revision_before: None,
        revision_after: None,
        outcome: "succeeded".into(),
        error: None,
        duration_ms: 12,
        undo_token: None,
        client: ClientInfo { name: "claude-code".into(), version: "1.0.0".into() },
    }
}

//#region 🔖️Redaction
#[test]
fn redact_input_replaces_only_sensitive_keys_case_insensitively() {
    let raw = serde_json::json!({ "Password": "hunter2", "dx": 1.0, "nested": { "API_KEY": "sk-live-xyz", "safe": "ok" } });
    let redacted = redact_input(&raw, SENSITIVE_KEYS);
    assert_eq!(redacted["Password"], "«redacted»");
    assert_eq!(redacted["dx"], 1.0);
    assert_eq!(redacted["nested"]["API_KEY"], "«redacted»");
    assert_eq!(redacted["nested"]["safe"], "ok");
}

/// 🔐️ THE security property: a raw secret must never reach a sink, through any field — proven by
/// building the event the way a real call site would (redact BEFORE constructing the event) and
/// asserting the appended, serialized event contains no trace of the raw value.
#[test]
fn sensitive_field_never_reaches_the_sink() {
    let raw_args = serde_json::json!({ "token": "sk-live-super-secret", "artifactId": "cad-1" });
    let redacted = redact_input(&raw_args, SENSITIVE_KEYS);
    let event = sample_event(redacted);

    let sink = InMemoryAuditSink::new();
    sink.append(&event).unwrap();

    let stored = sink.events();
    assert_eq!(stored.len(), 1);
    let serialized = serde_json::to_string(&stored[0]).unwrap();
    assert!(!serialized.contains("sk-live-super-secret"), "raw secret leaked into the sink: {serialized}");
    assert!(serialized.contains("«redacted»"));
    assert!(serialized.contains("cad-1"), "non-sensitive fields must still pass through");
}

#[test]
fn hash_input_is_deterministic_for_the_same_value() {
    let value = serde_json::json!({ "a": 1, "b": "two" });
    assert_eq!(hash_input(&value), hash_input(&value));
    let other = serde_json::json!({ "a": 2 });
    assert_ne!(hash_input(&value), hash_input(&other));
}
//#endregion 🔖️Redaction

//#region 🔖️InMemorySink
#[test]
fn in_memory_sink_preserves_append_order() {
    let sink = InMemoryAuditSink::new();
    sink.append(&sample_event(serde_json::json!({}))).unwrap();
    let mut second = sample_event(serde_json::json!({}));
    second.invocation_id = "inv-2".into();
    sink.append(&second).unwrap();
    let events = sink.events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].invocation_id, "inv-1");
    assert_eq!(events[1].invocation_id, "inv-2");
}
//#endregion 🔖️InMemorySink

//#region 🔖️FileSink
fn scratch_dir(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!("semio-mcp-audit-test-{label}-{}-{}", std::process::id(), framework_hash::hash_bytes(label.as_bytes())))
}

#[test]
fn file_sink_appends_one_json_line_per_event_and_creates_its_directory() {
    let directory = scratch_dir("append");
    let sink = FileAuditSink::new(&directory).unwrap();
    sink.append(&sample_event(serde_json::json!({}))).unwrap();
    let mut second = sample_event(serde_json::json!({}));
    second.invocation_id = "inv-2".into();
    sink.append(&second).unwrap();

    let contents = fs::read_to_string(sink.path()).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2);
    for line in &lines {
        let parsed: AgentAuditEvent = serde_json::from_str(line).unwrap();
        assert!(!parsed.invocation_id.is_empty());
    }
    fs::remove_dir_all(&directory).ok();
}

#[test]
fn default_audit_dir_ends_with_the_frozen_path_suffix() {
    let dir = default_audit_dir();
    assert!(dir.ends_with(std::path::Path::new(".semio").join("agent").join("audit")));
}
//#endregion 🔖️FileSink
