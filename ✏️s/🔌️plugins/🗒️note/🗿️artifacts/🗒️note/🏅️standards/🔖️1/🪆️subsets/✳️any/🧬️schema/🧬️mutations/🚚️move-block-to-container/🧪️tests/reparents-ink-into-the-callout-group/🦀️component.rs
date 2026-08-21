//! 🧪️ `move-block-to-container` fixture — `reparents-ink-into-the-callout-group`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{block_bounds, find_block, find_block_location};
use crate::artifacts::note::{NoteDiff, NoteSnapshot};
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

/// ▶️ `move-block-to-container` emits a `removed` id AND an `added` entry in ONE diff; apply runs removals before additions, so the block is lifted and re-placed atomically.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("move-block-to-container applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "move-block-to-container/reparents-ink-into-the-callout-group: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse re-issues `move-block-to-container` with the base's own `(parent_id, index)` from `find_block_location`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("move-block-to-container applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("move-block-to-container inverse step applies");
    }
    assert_eq!(snapshot, base, "move-block-to-container/reparents-ink-into-the-callout-group: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-block-to-container/reparents-ink-into-the-callout-group: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-block-to-container/reparents-ink-into-the-callout-group: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists, the container is a real group, and it is not the block itself, so neither the `mutation.target-missing` error nor the `mutation.invariant` fatal fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "move-block-to-container/reparents-ink-into-the-callout-group: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "move-block-to-container/reparents-ink-into-the-callout-group: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("move-block-to-container/reparents-ink-into-the-callout-group: declared applied but the diff would not apply");
}

/// 🔺️ A `removed` id AND an `added` entry in the SAME delta — the reparent is one atomic sparse change, and the added entry carries the block value verbatim from the base.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-block-to-container/reparents-ink-into-the-callout-group: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `move-block-to-container` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-block-to-container/reparents-ink-into-the-callout-group: committed diff JSON is not canonical");
}

/// 🩹 The committed remove+add delta carries `before` to `after` in one apply (removals run before additions).
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-block-to-container/reparents-ink-into-the-callout-group: committed diff did not carry before to after");
}

/// 🚚 The block changes PARENT, not coordinates: it leaves the root, enters the group at index 0 ahead of the existing child, and keeps its own x/y.
#[semio_framework_async_macros::async_test]
async fn block_changes_parent_at_an_index_without_moving_in_space() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("move-block-to-container applies");
    assert_eq!(find_block_location(&base.blocks, "blk-ink"), Some((None, 1)), "move-block-to-container/reparents-ink-into-the-callout-group: blk-ink must start at the document root");
    assert_eq!(find_block_location(&applied.blocks, "blk-ink"), Some((Some("blk-group".to_string()), 0)), "blk-ink must end up as the group's first child");
    assert_eq!(find_block_location(&applied.blocks, "blk-nested"), Some((Some("blk-group".to_string()), 1)), "the group's existing child must be pushed right by the index-0 insertion");
    assert_eq!(applied.blocks.len(), base.blocks.len() - 1, "the root list must lose exactly the reparented block");
    assert_eq!(block_bounds(find_block(&applied.blocks, "blk-ink").expect("the moved block exists")), block_bounds(find_block(&base.blocks, "blk-ink").expect("the base block exists")), "reparenting must not move the block in space");
}
