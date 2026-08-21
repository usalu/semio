//! 🧪️ `edit-block-ink-stroke` fixture — `redraws-the-sketch-polyline`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{block_bounds, find_block};
use crate::artifacts::note::{NoteBlockNode, NoteDiff, NoteSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> NoteSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> NoteSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> NoteMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `edit-block-ink-stroke` emits ONE whole-block `patched` entry that rewrites `points` AND the whole bounding box in a single atomic step.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("edit-block-ink-stroke applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "edit-block-ink-stroke/redraws-the-sketch-polyline: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse re-issues `edit-block-ink-stroke` carrying the base ink block's own prior points and box.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("edit-block-ink-stroke applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("edit-block-ink-stroke inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-block-ink-stroke/redraws-the-sketch-polyline: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-block-ink-stroke/redraws-the-sketch-polyline: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-block-ink-stroke/redraws-the-sketch-polyline: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists AND is an ink block, and points/box genuinely differ, so neither the `mutation.target-missing` error nor the `mutation.no-op` warn (which compares all five fields at once) fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "edit-block-ink-stroke/redraws-the-sketch-polyline: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "edit-block-ink-stroke/redraws-the-sketch-polyline: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("edit-block-ink-stroke/redraws-the-sketch-polyline: declared applied but the diff would not apply");
}

/// 🔺️ ONE `blocks.patched` entry carrying the new `points` AND the new box together — the atomicity is structural: there is no second entry to tear against.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-block-ink-stroke/redraws-the-sketch-polyline: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `edit-block-ink-stroke` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-block-ink-stroke/redraws-the-sketch-polyline: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`patched` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-block-ink-stroke/redraws-the-sketch-polyline: committed diff did not carry before to after");
}

/// 🎨 Geometry and bounding box move ATOMICALLY in one operation — a 2-point stroke becomes a 3-point polyline and the box retightens around it — while the stroke width is a different leaf's concern.
#[semio_framework_async_macros::async_test]
async fn points_and_bounding_box_are_rewritten_atomically() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("edit-block-ink-stroke applies");
    let NoteBlockNode::Ink { points: before_points, .. } = find_block(&base.blocks, "blk-ink").expect("the base ink block exists") else {
        panic!("edit-block-ink-stroke/redraws-the-sketch-polyline: the base block must be an ink block");
    };
    assert_eq!(before_points.len(), 2, "edit-block-ink-stroke/redraws-the-sketch-polyline: the base stroke must start as a 2-point segment");
    let NoteBlockNode::Ink { points, stroke_width, .. } = find_block(&applied.blocks, "blk-ink").expect("the ink block survives") else {
        panic!("edit-block-ink-stroke must not change the block's kind");
    };
    assert_eq!(points, &vec![[0.0, 0.0], [10.0, 4.0], [20.0, 0.0]], "the whole point list must be replaced by the addressed polyline");
    assert_eq!(*stroke_width, 2.0, "the stroke WIDTH belongs to change-block-ink-width, not to this leaf");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-ink").expect("the ink block exists")), (25.0, 150.0, 20.0, 4.0), "the bounding box is rewritten in the SAME atomic step as the geometry");
}
