//! 🧪️ `duplicate-blocks` fixture — `📌️copies-ink-and-table-with-shifting-indices`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::note::schema::find_block_location;
use crate::artifacts::note::schema::mutations::{apply_note_mutation, inverse_note_mutation, NoteMutation};
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

/// ▶️ `duplicate-blocks` emits ONE `added` entry per (source, copy) pair, each pinned to that source's own base index + 1 — the entries are then inserted in order, so later indices are read against an already-grown list.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("duplicate-blocks applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "duplicate-blocks/copies-ink-and-table-with-shifting-indices: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse is a single `delete-blocks` naming every copy's id.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("duplicate-blocks applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("duplicate-blocks inverse step applies");
    }
    assert_eq!(snapshot, base, "duplicate-blocks/copies-ink-and-table-with-shifting-indices: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "duplicate-blocks/copies-ink-and-table-with-shifting-indices: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "duplicate-blocks/copies-ink-and-table-with-shifting-indices: committed mutation JSON is not canonical");
}

/// 🎯️ Both sources exist and both copy ids are fresh, so none of the `mutation.duplicate-id` fatal, `mutation.target-missing` error or `mutation.partial` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "duplicate-blocks/copies-ink-and-table-with-shifting-indices: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "duplicate-blocks/copies-ink-and-table-with-shifting-indices: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("duplicate-blocks/copies-ink-and-table-with-shifting-indices: declared applied but the diff would not apply");
}

/// 🔺️ Two `blocks.added` entries, each pinned to its own source's BASE index + 1 (2 and 3) — the committed indices are what make the sequential-insert skew reproducible.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "duplicate-blocks/copies-ink-and-table-with-shifting-indices: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `duplicate-blocks` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "duplicate-blocks/copies-ink-and-table-with-shifting-indices: committed diff JSON is not canonical");
}

/// 🩹 Applying both committed `added` entries in order carries `before` to `after`, skew included.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "duplicate-blocks/copies-ink-and-table-with-shifting-indices: committed diff did not carry before to after");
}

/// 👥 Indices are computed against the BASE but applied sequentially, so the second copy lands ahead of its own source — the observable batch-insert skew this leaf has and `duplicate-block` cannot.
#[semio_framework_async_macros::async_test]
async fn second_copy_lands_ahead_of_its_own_source_from_index_skew() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("duplicate-blocks applies");
    assert_eq!(find_block_location(&base.blocks, "blk-ink"), Some((None, 1)), "duplicate-blocks/copies-ink-and-table-with-shifting-indices: blk-ink must start at root index 1");
    assert_eq!(find_block_location(&base.blocks, "blk-table"), Some((None, 2)), "duplicate-blocks/copies-ink-and-table-with-shifting-indices: blk-table must start at root index 2");
    assert_eq!(applied.blocks.len(), base.blocks.len() + 2, "duplicate-blocks adds exactly one copy per source in ONE operation");
    assert_eq!(find_block_location(&applied.blocks, "blk-ink-copy"), Some((None, 2)), "the first copy lands at its source index + 1");
    assert_eq!(find_block_location(&applied.blocks, "blk-table-copy"), Some((None, 3)), "the second copy uses its BASE index + 1 against an already-grown list");
    assert_eq!(find_block_location(&applied.blocks, "blk-table"), Some((None, 4)), "so the second copy ends up ahead of its own source — the batch-insert skew this leaf owns");
}
