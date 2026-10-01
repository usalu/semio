use super::*;
use crate::editor::note::unit_tests::context::{dispatch, note_app, render, select_blocks};
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

/// 🖊️ One `inkApplyEvents` dispatch of `events` in `phase` (absent = one-shot), with an optional drag record.
fn ink(events: serde_json::Value, phase: Option<&str>, gesture: Option<serde_json::Value>) -> NoteCommand {
    NoteCommand::InkApplyEvents(InkApplyEvents { events_json: events.to_string(), phase: phase.map(str::to_string), reason: None, gesture_json: gesture.map(|gesture| gesture.to_string()), select_ids: None })
}

/// 🧯️ A host abort of the open gesture with `reason`.
fn abort(reason: &str) -> NoteCommand {
    NoteCommand::InkApplyEvents(InkApplyEvents { events_json: String::new(), phase: Some("abort".into()), reason: Some(reason.into()), gesture_json: None, select_ids: None })
}

// 🧾️ The history rows that carry a tool transaction (plain comment: rustdoc cannot document a macro).
macro_rules! transaction_rows {
    ($app:expr) => {
        $app.history_snapshot().await.expect("history").upserts.into_iter().filter(|row| row.transaction.is_some()).collect::<Vec<_>>()
    };
}

/// ✏️ `block` moved to `(x, y)` (a text block in these laws).
fn moved(block: &NoteBlockNode, x: f64, y: f64) -> NoteBlockNode {
    let mut moved = block.clone();
    if let NoteBlockNode::Text { x: block_x, y: block_y, .. } = &mut moved {
        (*block_x, *block_y) = (x, y);
    }
    moved
}

#[test]
fn unknown_phases_and_malformed_gesture_records_fault() {
    assert_eq!(NoteInkPhase::parse(None, None).unwrap(), NoteInkPhase::Once);
    assert_eq!(NoteInkPhase::parse(Some("abort"), Some("captureLost")).unwrap(), NoteInkPhase::Abort(semio_framework_tool_machine::ToolAbortReason::CaptureLost));
    assert_eq!(NoteInkPhase::parse(Some("abort"), None).unwrap(), NoteInkPhase::Abort(semio_framework_tool_machine::ToolAbortReason::Tool));
    for (phase, reason) in [(Some("begin"), None), (Some("live"), None), (Some("atomic"), None), (Some("abort"), Some("bogus"))] {
        assert!(NoteInkPhase::parse(phase, reason).is_err(), "{phase:?} {reason:?}");
    }
    assert!(note_gesture_yields("{\"kind\":\"scale\",\"ids\":[\"a\"],\"dx\":1,\"dy\":1}").is_err());
    assert!(note_gesture_yields("{\"kind\":\"drag\",\"ids\":[\"a\"],\"dx\":\"1\",\"dy\":1}").is_err());
    assert_eq!(note_gesture_yields("{\"kind\":\"drag\",\"ids\":[\"a\",\"a\"],\"dx\":2,\"dy\":3}").unwrap(), vec![semio_framework_tool_machine::ToolYield::upsert(NOTE_INK_GESTURE_KEY, crate::schema::mutations::drag_blocks(vec!["a".into()], 2.0, 3.0))]);
    assert_eq!(note_gesture_yields("{\"kind\":\"drag\",\"ids\":[\"a\"],\"dx\":0,\"dy\":0}").unwrap(), vec![semio_framework_tool_machine::ToolYield::retract(NOTE_INK_GESTURE_KEY)]);
}

/// 🌊️ LAW (design §5): every `stream` tick lands in the window's ONE open transaction — the window previews it, the
/// document never moves, no history row appears — and the `commit` publishes it as ONE edit, one row, one transaction
/// whose leaf is the net creation; one undo takes the whole gesture back.
#[semio_framework_async_macros::async_test]
async fn a_streamed_gesture_is_one_transaction_and_one_undo_step() {
    let mut app = note_app().await;
    let mut ids = crate::schema::NoteIdOwner::new("ink-test", 0);
    let block = create_block_by_kind(&mut ids, "text", 10.0, 10.0);
    let new_id = block_id(&block).to_string();
    dispatch(&mut app, ink(json!([{ "operation": "addBlock", "block": ink_wire_block(&block), "parentId": null, "index": null }]), Some("stream"), None)).await;
    for x in [20.0, 30.0, 40.0] {
        dispatch(&mut app, ink(json!([{ "operation": "updateBlock", "blockId": new_id, "block": ink_wire_block(&moved(&block, x, 10.0)) }]), Some("stream"), None)).await;
        assert!(app.snapshot().expect("snapshot").blocks.is_empty(), "a stream tick never touches the committed document");
        assert!(transaction_rows!(app).is_empty(), "a stream tick never writes history");
    }
    assert!(render(&mut app, crate::editor::note::modes::edit::windows::composite::NOTE_PLAY_BODY_COMPOSITE).await.contains(&new_id), "the gesture's window paints committed ⊕ provisional");
    dispatch(&mut app, ink(json!([]), Some("commit"), None)).await;
    let committed = app.snapshot().expect("snapshot");
    assert_eq!(committed.blocks, vec![moved(&block, 40.0, 10.0)], "the commit publishes the net gesture");
    let rows = transaction_rows!(app);
    assert_eq!(rows.len(), 1, "one gesture is one history row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().unwrap();
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.note.note@1/*#editor#inkApplyEvents", "{transaction:?}");
    assert_eq!(rows[0].op_count, 1, "one net create-block, not one op per tick");
    semio_framework_plugin::artifact_app_laws::settle_history_verb(&mut *app, "undo", 1).await;
    assert!(app.snapshot().expect("snapshot").blocks.is_empty(), "a single undo erases the whole gesture");
}

/// 🧯️ LAW (design §5): a host abort drops the open gesture with zero trace — no document change, no history row, no
/// preview — and the next gesture opens fresh and publishes exactly once.
#[semio_framework_async_macros::async_test]
async fn an_aborted_gesture_leaves_zero_trace_and_a_fresh_gesture_publishes_once() {
    let surface_behavior: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🎬️surface-behavior/🔣️.json"))).expect("shared surface behavior fixture");
    let law = surface_behavior["cases"].as_array().unwrap().iter().find(|entry| entry["id"] == "note-ink-canvas").expect("Note surface behavior");
    assert_eq!(law["terminalPolicy"], "cancelled-action-discards-draft");
    assert_eq!(law["publishedOnCancel"], json!([{ "action": "inkApplyEvents", "phase": "abort", "cancelled": null }]));
    assert_eq!(law["artifactAfterCancel"], "unchanged");
    assert_eq!(law["freshGestureResult"], "one-artifact-publication");

    let mut app = note_app().await;
    let mut ids = crate::schema::NoteIdOwner::new("ink-cancel-test", 0);
    let block = create_block_by_kind(&mut ids, "text", 32.0, 40.0);
    let id = block_id(&block).to_string();
    dispatch(&mut app, ink(json!([{ "operation": "addBlock", "block": ink_wire_block(&block), "parentId": null, "index": null }]), Some("stream"), None)).await;
    dispatch(&mut app, ink(json!([{ "operation": "updateBlock", "blockId": id, "block": ink_wire_block(&moved(&block, 48.0, 52.0)) }]), Some("stream"), None)).await;
    dispatch(&mut app, abort("captureLost")).await;
    assert!(app.snapshot().expect("snapshot").blocks.is_empty(), "an aborted gesture never reaches the document");
    assert!(transaction_rows!(app).is_empty(), "an aborted gesture never reaches history");
    assert!(!render(&mut app, crate::editor::note::modes::edit::windows::composite::NOTE_PLAY_BODY_COMPOSITE).await.contains(&id), "the preview is cleared");
    let commit = dispatch(&mut app, ink(json!([]), Some("commit"), None)).await;
    assert!(commit.mutations.is_empty(), "a commit after the abort finds no open gesture");

    let fresh = create_block_by_kind(&mut ids, "text", 72.0, 80.0);
    let fresh_id = block_id(&fresh).to_string();
    dispatch(&mut app, ink(json!([{ "operation": "addBlock", "block": ink_wire_block(&fresh), "parentId": null, "index": null }]), Some("stream"), None)).await;
    dispatch(&mut app, ink(json!([{ "operation": "updateBlock", "blockId": fresh_id, "block": ink_wire_block(&moved(&fresh, 88.0, 96.0)) }]), Some("commit"), None)).await;
    assert_eq!(app.snapshot().expect("snapshot").blocks, vec![moved(&fresh, 88.0, 96.0)]);
    assert_eq!(transaction_rows!(app).len(), 1, "the fresh gesture is one publication");
}

/// 🤏️ LAW (design §5): a block drag streams the host's gesture record and commits ONE relative `drag-blocks` leaf —
/// the net offset over every dragged block — labelled from the leaf in English and German; two gestures are two
/// transactions.
#[semio_framework_async_macros::async_test]
async fn a_block_drag_commits_one_relative_drag_blocks_leaf() {
    let mut app = note_app().await;
    let mut ids = crate::schema::NoteIdOwner::new("ink-drag-test", 0);
    let first = create_block_by_kind(&mut ids, "text", 0.0, 0.0);
    let second = create_block_by_kind(&mut ids, "text", 100.0, 0.0);
    let (a, b) = (block_id(&first).to_string(), block_id(&second).to_string());
    dispatch(&mut app, ink(json!([{ "operation": "addBlock", "block": ink_wire_block(&first) }, { "operation": "addBlock", "block": ink_wire_block(&second) }]), None, None)).await;
    let placed = app.snapshot().expect("snapshot");
    for (dx, dy) in [(10.0, 5.0), (20.0, 10.0)] {
        dispatch(&mut app, ink(json!([]), Some("stream"), Some(json!({ "kind": "drag", "ids": [a, b], "dx": dx, "dy": dy })))).await;
        assert_eq!(app.snapshot().expect("snapshot"), placed, "a drag tick never moves the committed document");
    }
    dispatch(&mut app, ink(json!([]), Some("commit"), Some(json!({ "kind": "drag", "ids": [a, b], "dx": 30.0, "dy": 15.0 })))).await;
    let after = app.snapshot().expect("snapshot");
    assert_eq!(after.blocks, vec![moved(&first, 30.0, 15.0), moved(&second, 130.0, 15.0)]);
    let rows = transaction_rows!(app);
    assert_eq!(rows.len(), 2, "the placement and the drag are two transactions: {rows:?}");
    assert_ne!(rows[0].transaction, rows[1].transaction);
    let drag = &rows[1];
    assert_eq!(drag.op_lines.len(), 1, "one relative leaf: {:?}", drag.op_lines);
    assert!(drag.op_lines[0].starts_with("drag-blocks"), "{:?}", drag.op_lines);
    assert_eq!(drag.label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 2 blocks by (30, 15)");
    assert_eq!(drag.label.resolve(protocol::Terminology::Native, protocol::Locale::De), "2 Blöcke um (30; 15) ziehen");
}

/// ⌨️ LAW (design §5): a keyboard nudge is ONE transaction of ONE `drag-blocks` over every unlocked selected block —
/// never one leaf per block — and a one-shot interrupting an open canvas gesture aborts it `captureLost`.
#[semio_framework_async_macros::async_test]
async fn a_nudge_is_one_drag_blocks_transaction_and_interrupts_an_open_gesture() {
    let mut app = note_app().await;
    let mut ids = crate::schema::NoteIdOwner::new("ink-nudge-test", 0);
    let first = create_block_by_kind(&mut ids, "text", 0.0, 0.0);
    let second = create_block_by_kind(&mut ids, "text", 50.0, 50.0);
    let (a, b) = (block_id(&first).to_string(), block_id(&second).to_string());
    dispatch(&mut app, ink(json!([{ "operation": "addBlock", "block": ink_wire_block(&first) }, { "operation": "addBlock", "block": ink_wire_block(&second) }]), None, None)).await;
    select_blocks(&mut app, &[a.as_str(), b.as_str()]).await;
    dispatch(&mut app, ink(json!([]), Some("stream"), Some(json!({ "kind": "drag", "ids": [a], "dx": 99.0, "dy": 99.0 })))).await;
    dispatch(&mut app, NoteCommand::NudgeSelectionRightFast(crate::editor::note::commands::nudge_selection_right_fast::NudgeSelectionRightFast {})).await;
    assert_eq!(app.snapshot().expect("snapshot").blocks, vec![moved(&first, 10.0, 0.0), moved(&second, 60.0, 50.0)], "the nudge moved both; the interrupted drag left nothing");
    dispatch(&mut app, ink(json!([]), Some("commit"), Some(json!({ "kind": "drag", "ids": [a], "dx": 99.0, "dy": 99.0 })))).await;
    assert_eq!(app.snapshot().expect("snapshot").blocks[0], moved(&first, 109.0, 99.0), "a commit after the interruption is a fresh one-shot drag");
    let rows = transaction_rows!(app);
    let nudge = rows.iter().find(|row| row.transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#nudgeSelectionRightFast"))).expect("the nudge row");
    assert_eq!(nudge.op_count, 1, "one drag-blocks over both selected blocks");
}
