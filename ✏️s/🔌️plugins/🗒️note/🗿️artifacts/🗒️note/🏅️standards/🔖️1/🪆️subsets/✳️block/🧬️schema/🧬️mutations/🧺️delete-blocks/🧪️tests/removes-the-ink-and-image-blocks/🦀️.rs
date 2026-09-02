//! 🧪️ `delete-blocks` fixture — `removes-the-ink-and-image-blocks`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{find_block, find_block_location};
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

/// ▶️ `delete-blocks` emits ONE `removed` list holding every addressed id that actually exists.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("delete-blocks applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-blocks/removes-the-ink-and-image-blocks: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse is a `create-block` PER removed id, ordered so the caller's reversal replays them lowest-index-first and each original absolute index stays valid.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("delete-blocks applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("delete-blocks inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-blocks/removes-the-ink-and-image-blocks: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-blocks/removes-the-ink-and-image-blocks: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-blocks/removes-the-ink-and-image-blocks: committed mutation JSON is not canonical");
}

/// 🎯️ Both ids exist, so neither the `mutation.target-missing` error nor the `mutation.partial` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "delete-blocks/removes-the-ink-and-image-blocks: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "delete-blocks/removes-the-ink-and-image-blocks: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("delete-blocks/removes-the-ink-and-image-blocks: declared applied but the diff would not apply");
}

/// 🔺️ Both ids land in ONE `blocks.removed` list, in the payload's own order — the batch verb collapses to a single delta, not two.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-blocks/removes-the-ink-and-image-blocks: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `delete-blocks` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-blocks/removes-the-ink-and-image-blocks: committed diff JSON is not canonical");
}

/// 🩹 The committed two-id `removed` delta carries `before` to `after` in one apply.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-blocks/removes-the-ink-and-image-blocks: committed diff did not carry before to after");
}

/// 🧺 Two NON-ADJACENT root blocks go in one operation — the batch verb, not two `delete-block`s — and the survivors keep their relative order.
#[semio_framework_async_macros::async_test]
async fn two_non_adjacent_blocks_go_in_one_batch() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("delete-blocks applies");
    assert_eq!(find_block_location(&base.blocks, "blk-ink"), Some((None, 1)), "delete-blocks/removes-the-ink-and-image-blocks: blk-ink must start at root index 1");
    assert_eq!(find_block_location(&base.blocks, "blk-image"), Some((None, 4)), "delete-blocks/removes-the-ink-and-image-blocks: blk-image must start at root index 4, non-adjacent to blk-ink");
    assert!(find_block(&applied.blocks, "blk-ink").is_none(), "blk-ink must be gone");
    assert!(find_block(&applied.blocks, "blk-image").is_none(), "blk-image must be gone");
    assert_eq!(applied.blocks.len(), base.blocks.len() - 2, "delete-blocks must shrink the root list by exactly two in ONE operation");
    assert_eq!(find_block_location(&applied.blocks, "blk-table"), Some((None, 1)), "survivors must close up in their original relative order");
    assert_eq!(inverse_note_mutation(&base, &mutation()).len(), 2, "the inverse must be one create-block per removed id");
}
