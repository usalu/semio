//! 🧪️ Actual-grant session byte retirement, final cache ownership, and strict lifecycle guards.

use super::*;

//#region 🧪️SessionRetirement
fn close(mut session: FlowEvalSession, maximum_bytes: usize) -> usize {
    use semio_framework_job::InteractiveJobCloseStep as Step;
    session.begin_close(); let mut released = 0;
    assert!(matches!(session.close_step(0, maximum_bytes), Step::Blocked));
    assert!(matches!(session.close_step(1, 0), Step::Blocked));
    for _ in 0..1_000_000 {
        match session.close_step(1, maximum_bytes) {
            Step::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= maximum_bytes); released += released_bytes; }
            Step::Complete => { assert!(session.terminal_is_empty()); return released; }
            Step::Blocked => panic!("positive session grant blocked"),
        }
    }
    panic!("session retirement did not reach terminal-empty")
}

#[test]
fn session_semantic_bytes_larger_than_production_grant_retire_exactly_across_workers() {
    let fixture = crate::os_pack::json::parse(include_str!("../🔣️session.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        let text = fixture.get("text").and_then(|v| v.get("text")).and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(fixture.get("text").and_then(|v| v.get("repeat")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        let preview = fixture.get("preview").and_then(|v| v.get("text")).and_then(crate::os_pack::json::Value::as_str).unwrap().repeat(fixture.get("preview").and_then(|v| v.get("repeat")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        let mut session = FlowEvalSession::new();
        session.eval_json = String::with_capacity(fixture.get("text").and_then(|v| v.get("reservedCapacity")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
        session.eval_json.push_str(&text);
        assert!(session.eval_json.capacity() > session.eval_json.len());
        session.preview_mesh_json_by_handle.insert("mesh".into(), preview.clone());
        session.pending_tessellate_by_hash.insert(1, "pending".into());
        session.live_geometry_handles.insert("geometry".into());
        session.previous_channels = Some(EvalChannels { outputs: BTreeMap::from([("output".into(), Dictionary::new().insert("label", NeuralValue::Atom(Atom::String(preview))))]), inputs: BTreeMap::new() });
        session.neural_cache().seed(1, Dictionary::new().insert("label", NeuralValue::Atom(Atom::String(text))));
        let released = std::thread::spawn(move || close(session, maximum_bytes)).join().unwrap();
        assert_eq!(released, fixture.get("expected").and_then(|v| v.get("releasedBytes")).and_then(crate::os_pack::json::Value::as_u64).unwrap() as usize);
    }
}

#[test]
fn empty_reserved_text_does_not_require_capacity_sized_credit() {
    let mut session = FlowEvalSession::new(); session.eval_json = String::with_capacity(65536);
    assert_eq!(close(session, 1), 2);
}

#[test]
fn live_session_drop_is_rejected_without_recursive_payload_destruction() {
    assert!(std::panic::catch_unwind(|| drop(FlowEvalSession::new())).is_err());
}
//#endregion 🧪️SessionRetirement
