//! 🧪️ Inline canvas events survive mutation replay and undo.
use super::*;
use crate::schema::mutations::inverse_note_mutation;
use protocol::OpBinary;

#[test]
fn inline_block_edits_replay_and_undo() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("inline-edits.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let decode = |value: &serde_json::Value| {
            decode_canvas_events(&serde_json::json!([{"operation":"addBlock","block":value}]).to_string()).unwrap().remove(0)
        };
        let NoteCanvasEvent::AddBlock { block: before, .. } = decode(&case["before"]) else { unreachable!() };
        let NoteCanvasEvent::AddBlock { block: after, .. } = decode(&case["after"]) else { unreachable!() };
        let base = NoteSnapshot { blocks: vec![before], ..NoteSnapshot::default() };
        let update = serde_json::json!([{"operation":"updateBlock","blockId":case["before"]["id"],"block":case["after"]}]);
        let events = decode_canvas_events(&update.to_string()).unwrap();
        let yields = note_event_yields(&base, &[], &events).unwrap();
        let [ToolYield::Upsert { .. }] = yields.as_slice() else {
            panic!("{} should publish one mutation: {yields:?}", case["mutation"]);
        };
        let ToolYield::Upsert { mutation, .. } = yields.into_iter().next().unwrap() else { unreachable!() };
        assert_eq!(semio_framework_value::ToValue::to_value(&mutation).get("mutation").and_then(semio_framework_value::DslValue::as_str), case["mutation"].as_str());
        let replayed = NoteMutation::decode_op(&mutation.encode_op().unwrap()).unwrap();
        let committed = apply_note_mutation(&base, &replayed).unwrap();
        assert_eq!(committed.blocks, vec![after]);
        assert!(note_event_yields(&committed, &[], &events).unwrap().is_empty());
        let mut restored = committed;
        for inverse in inverse_note_mutation(&base, &replayed).unwrap().into_iter().rev() {
            restored = apply_note_mutation(&restored, &inverse).unwrap();
        }
        assert_eq!(restored, base);
    }
}
