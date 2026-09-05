//! 🧪️ `duplicate-block` fixture — `🧩️copies-the-math-block-right-after-its-source`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
use crate::artifacts::note::schema::{block_name, find_block, find_block_location};
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

/// ▶️ `duplicate-block` places the copy at the SOURCE's own `(parent_id, index + 1)`, taken from the base — the payload carries no position at all.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("duplicate-block applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "duplicate-block/copies-the-math-block-right-after-its-source: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse is `delete-block` on the copy's id.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("duplicate-block applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("duplicate-block inverse step applies");
    }
    assert_eq!(snapshot, base, "duplicate-block/copies-the-math-block-right-after-its-source: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "duplicate-block/copies-the-math-block-right-after-its-source: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "duplicate-block/copies-the-math-block-right-after-its-source: committed mutation JSON is not canonical");
}

/// 🎯️ The source exists and the copy's id is fresh, so neither the `mutation.target-missing` error nor the `mutation.duplicate-id` fatal fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "duplicate-block/copies-the-math-block-right-after-its-source: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "duplicate-block/copies-the-math-block-right-after-its-source: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("duplicate-block/copies-the-math-block-right-after-its-source: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.added` entry whose index is the SOURCE's base index + 1; `removed` stays empty, so the source is provably untouched.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "duplicate-block/copies-the-math-block-right-after-its-source: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `duplicate-block` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "duplicate-block/copies-the-math-block-right-after-its-source: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`added` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "duplicate-block/copies-the-math-block-right-after-its-source: committed diff did not carry before to after");
}

/// 🎯 The copy lands immediately AFTER its source (source index + 1), keeping the source in place, and carries the payload's own new identity.
#[semio_framework_async_macros::async_test]
async fn copy_lands_immediately_after_its_source() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("duplicate-block applies");
    assert_eq!(find_block_location(&base.blocks, "blk-math"), Some((None, 3)), "duplicate-block/copies-the-math-block-right-after-its-source: the source must start at root index 3");
    assert_eq!(find_block_location(&applied.blocks, "blk-math"), Some((None, 3)), "the source must stay exactly where it was");
    assert_eq!(find_block_location(&applied.blocks, "blk-math-copy"), Some((None, 4)), "the copy must land at source index + 1");
    assert_eq!(block_name(find_block(&applied.blocks, "blk-math-copy").expect("the copy exists")), "Formula copy", "the copy carries the identity the payload named");
    assert_eq!(applied.blocks.len(), base.blocks.len() + 1, "duplicate-block adds exactly one block");
}
