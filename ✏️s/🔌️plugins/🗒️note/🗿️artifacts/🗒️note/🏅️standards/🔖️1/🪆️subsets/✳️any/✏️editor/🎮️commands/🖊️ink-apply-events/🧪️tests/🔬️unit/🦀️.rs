use super::*;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::NoteCommand;
use crate::schema::{block_id, create_block_by_kind};
use crate::{NoteCamera, NoteSnapshot};
use semio_framework_plugin::PluginApp;
use serde_json::json;

/// 🖋️ One block spelled the way the ink-canvas host sends it — taken from the note's own canvas
/// projection, so the test pins the round trip host wire → note block.
fn ink_wire_block(block: &NoteBlockNode) -> serde_json::Value {
    let snapshot = NoteSnapshot { blocks: vec![block.clone()], ..crate::schema::empty_note_snapshot() };
    let document: serde_json::Value = serde_json::from_str(&crate::note_canvas_document_json(&snapshot, &NoteCamera::default())).expect("canvas JSON");
    assert_eq!(document["schema"], "ink.document");
    document["blocks"][0].clone()
}

#[semio_framework_async_macros::async_test]
async fn ink_wire_text_block_round_trips_into_the_composed_text_record() {
    let mut ids = crate::schema::NoteIdOwner::new("ink-wire-test", 0);
    let block = create_block_by_kind(&mut ids, "text", 10.0, 10.0);
    let wire = ink_wire_block(&block);
    assert!(wire.get("content").is_none() && wire["paragraphs"].is_array(), "the host reads bare paragraphs: {wire}");
    let mut value = dsl::os_pack::json_to_dsl_value(&dsl::os_pack::json::parse(&wire.to_string()).expect("wire JSON"));
    crate::note_block_value_from_ink_wire(&mut value).expect("wire block");
    assert_eq!(<NoteBlockNode as dsl::FromValue>::from_value(value).expect("note block"), block);
}

#[semio_framework_async_macros::async_test]
async fn malformed_ink_event_batches_fault_instead_of_vanishing() {
    assert!(decode_canvas_events("[{\"mutation\":\"removeBlock\",\"blockId\":\"b1\"}]").is_err(), "a batch keyed by the retired `mutation` tag must fault");
    assert!(decode_canvas_events("not json").is_err());
    assert_eq!(decode_canvas_events("[{\"operation\":\"removeBlock\",\"blockId\":\"b1\"}]").expect("host batch").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn gesture_begin_live_commit_produces_single_undo_step() {
    let mut app = note_app().await;
    let mut ids = crate::schema::NoteIdOwner::new("ink-test", 0);
    let block = create_block_by_kind(&mut ids, "text", 10.0, 10.0);
    let new_id = block_id(&block).to_string();

    let begin_events = json!([
        { "operation": "addBlock", "block": ink_wire_block(&block), "parentId": null, "index": null }
    ])
    .to_string();
    dispatch(&mut app, NoteCommand::InkApplyEvents(InkApplyEvents { events_json: begin_events, phase: "begin".into(), select_ids: Some(vec![new_id.clone()]) })).await;
    assert_eq!(app.snapshot().expect("snapshot").blocks.len(), 1);

    for x in [20.0, 30.0, 40.0] {
        let mut moved = block.clone();
        if let NoteBlockNode::Text { x: block_x, .. } = &mut moved {
            *block_x = x;
        }
        let live_events = json!([
            { "operation": "updateBlock", "blockId": new_id, "block": ink_wire_block(&moved) }
        ])
        .to_string();
        dispatch(&mut app, NoteCommand::InkApplyEvents(InkApplyEvents { events_json: live_events, phase: "live".into(), select_ids: None })).await;
    }
    assert_eq!(app.snapshot().expect("snapshot").blocks.len(), 1);

    // Commit with no further change emits no operation — the gesture is already recorded.
    let commit = dispatch(&mut app, NoteCommand::InkApplyEvents(InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None })).await;
    assert!(commit.mutations.is_empty(), "a no-operation commit must not create an edit");
    assert_eq!(app.snapshot().expect("snapshot").blocks.len(), 1);

    // The whole begin+live gesture coalesced into ONE undoable edit.
    app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("undo");
    assert!(app.snapshot().expect("snapshot").blocks.is_empty(), "a single undo should erase the whole gesture");
}

#[semio_framework_async_macros::async_test]
async fn gesture_with_no_changes_creates_no_edit() {
    let mut app = note_app().await;
    dispatch(&mut app, NoteCommand::InkApplyEvents(InkApplyEvents { events_json: "[]".into(), phase: "begin".into(), select_ids: None })).await;
    dispatch(&mut app, NoteCommand::InkApplyEvents(InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None })).await;
    let undo = app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("undo");
    assert!(undo.events.is_empty(), "no gesture edit should exist to undo");
}
