//! 🧪️ `change-block-ink-width` fixture — `thickens-the-sketch-stroke`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::find_block;
use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::{NoteBlockNode, NoteDiff, NoteSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-block-ink-width` emits ONE whole-block `patched` entry whose only changed field is `stroke_width`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("change-block-ink-width applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-block-ink-width/thickens-the-sketch-stroke: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse re-issues `change-block-ink-width` with the base ink block's own prior width.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("change-block-ink-width applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("change-block-ink-width inverse step applies");
    }
    assert_eq!(snapshot, base, "change-block-ink-width/thickens-the-sketch-stroke: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-block-ink-width/thickens-the-sketch-stroke: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-block-ink-width/thickens-the-sketch-stroke: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists AND is an ink block, and 6.0 differs from 2.0, so neither the `mutation.target-missing` error (absent or non-ink) nor the `mutation.no-op` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "change-block-ink-width/thickens-the-sketch-stroke: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "change-block-ink-width/thickens-the-sketch-stroke: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("change-block-ink-width/thickens-the-sketch-stroke: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.patched` entry whose `blockJson` keeps the original `points` and `color`; `pencilWidth` stays `None`, so the block edit cannot leak into the tool setting.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-block-ink-width/thickens-the-sketch-stroke: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `change-block-ink-width` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-block-ink-width/thickens-the-sketch-stroke: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`patched` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-block-ink-width/thickens-the-sketch-stroke: committed diff did not carry before to after");
}

/// 🖊️ The DRAWN stroke's own width changes; its point list, colour and the document's pencil tool setting are all left alone.
#[semio_framework_async_macros::async_test]
async fn drawn_stroke_width_changes_without_touching_points_or_the_tool() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("change-block-ink-width applies");
    let NoteBlockNode::Ink { stroke_width: before_width, points: before_points, .. } = find_block(&base.blocks, "blk-ink").expect("the base ink block exists") else {
        panic!("change-block-ink-width/thickens-the-sketch-stroke: the base block must be an ink block");
    };
    assert_eq!(*before_width, 2.0, "change-block-ink-width/thickens-the-sketch-stroke: the base stroke must start at 2.0");
    let NoteBlockNode::Ink { stroke_width, points, color, .. } = find_block(&applied.blocks, "blk-ink").expect("the ink block survives") else {
        panic!("change-block-ink-width must not change the block's kind");
    };
    assert_eq!(*stroke_width, 6.0, "change-block-ink-width/thickens-the-sketch-stroke: the stroke must thicken to 6.0");
    assert_eq!(points, before_points, "thickening a stroke must not redraw its geometry");
    assert_eq!(*color, [0.0, 0.0, 0.0, 1.0], "thickening a stroke must not recolour it");
    assert_eq!(applied.pencil_width, Some(3.0), "the document's pencil TOOL width is a separate setting");
}
