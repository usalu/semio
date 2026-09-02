//! 🧪️ `drag-blocks` fixture — `nudges-ink-and-the-whole-group-subtree`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{block_bounds, find_block};
use crate::artifacts::note::{NoteDiff, NoteSnapshot};
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

/// ▶️ `drag-blocks` emits ONE `patched` entry per addressed block, each holding the offset whole-block value produced by `offset_block_tree`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("drag-blocks applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "drag-blocks/nudges-ink-and-the-whole-group-subtree: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse is the same `drag-blocks` with `(-dx, -dy)` — no snapshot lookup at all.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("drag-blocks applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("drag-blocks inverse step applies");
    }
    assert_eq!(snapshot, base, "drag-blocks/nudges-ink-and-the-whole-group-subtree: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "drag-blocks/nudges-ink-and-the-whole-group-subtree: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "drag-blocks/nudges-ink-and-the-whole-group-subtree: committed mutation JSON is not canonical");
}

/// 🎯️ Both ids exist, so neither the `mutation.target-missing` error nor the `mutation.partial` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "drag-blocks/nudges-ink-and-the-whole-group-subtree: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "drag-blocks/nudges-ink-and-the-whole-group-subtree: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("drag-blocks/nudges-ink-and-the-whole-group-subtree: declared applied but the diff would not apply");
}

/// 🔺️ Two `blocks.patched` entries, one per addressed id; the group's `blockJson` already contains its offset child, so the subtree recursion is visible in the delta itself.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "drag-blocks/nudges-ink-and-the-whole-group-subtree: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `drag-blocks` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "drag-blocks/nudges-ink-and-the-whole-group-subtree: committed diff JSON is not canonical");
}

/// 🩹 The committed two-entry `patched` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "drag-blocks/nudges-ink-and-the-whole-group-subtree: committed diff did not carry before to after");
}

/// 🤏 A RELATIVE offset, applied to several blocks at once, that recurses into a dragged group's children — the nested badge moves with its parent.
#[semio_framework_async_macros::async_test]
async fn relative_offset_recurses_into_the_dragged_group_subtree() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("drag-blocks applies");
    assert_eq!(block_bounds(find_block(&base.blocks, "blk-ink").expect("the base ink block exists")), (20.0, 160.0, 80.0, 40.0), "drag-blocks/nudges-ink-and-the-whole-group-subtree: the base geometry must be the one this case was derived from");
    let (ink_x, ink_y, ink_w, ink_h) = block_bounds(find_block(&applied.blocks, "blk-ink").expect("the dragged ink block exists"));
    assert_eq!((ink_x, ink_y), (32.0, 152.0), "the offset must be RELATIVE: (20, 160) + (12, -8)");
    assert_eq!((ink_w, ink_h), (80.0, 40.0), "dragging must never resize a block");
    let (group_x, group_y, ..) = block_bounds(find_block(&applied.blocks, "blk-group").expect("the dragged group exists"));
    assert_eq!((group_x, group_y), (352.0, 192.0), "the group itself takes the same offset");
    let (badge_x, badge_y, ..) = block_bounds(find_block(&applied.blocks, "blk-nested").expect("the nested badge exists"));
    assert_eq!((badge_x, badge_y), (362.0, 202.0), "a dragged group carries its children: the offset recurses into the subtree");
}
