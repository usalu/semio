use super::*;
use crate::editor::note::unit_tests::context::{dispatch, note_app};
use crate::editor::note::NoteCommand;
use crate::schema::{block_id, create_block_by_kind};
use semio_framework_plugin::PluginApp;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn gesture_begin_live_commit_produces_single_undo_step() {
    let mut app = note_app().await;
    let mut ids = crate::schema::NoteIdOwner::new("ink-test", 0);
    let block = create_block_by_kind(&mut ids, "text", 10.0, 10.0);
    let new_id = block_id(&block).to_string();

    let begin_events = json!([
        { "mutation": "addBlock", "block": serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&block)).expect("block JSON oracle"), "parentId": null, "index": null }
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
            { "mutation": "updateBlock", "blockId": new_id, "block": serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&moved)).expect("moved JSON oracle") }
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
