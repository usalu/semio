//! 🧪️ `remove-table-row` fixture — `🌱️drops-the-trailing-blank-row`.
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

/// ▶️ `remove-table-row` emits ONE whole-block `patched` entry that pops the LAST row — the row index is never a payload field.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("remove-table-row applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "remove-table-row/drops-the-trailing-blank-row: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse is `insert-table-row`, which re-appends a blank row of the current width; the fixture's trailing row is blank precisely so that round-trips.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("remove-table-row applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("remove-table-row inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-table-row/drops-the-trailing-blank-row: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-table-row/drops-the-trailing-blank-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-table-row/drops-the-trailing-blank-row: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists, is a table, and holds more than one row, so neither the `mutation.target-missing` error nor the one-row-floor `mutation.no-op` warn fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "remove-table-row/drops-the-trailing-blank-row: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "remove-table-row/drops-the-trailing-blank-row: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("remove-table-row/drops-the-trailing-blank-row: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.patched` entry whose `blockJson` holds the single surviving row — the popped row is the LAST one, which the committed delta makes checkable.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-table-row/drops-the-trailing-blank-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `remove-table-row` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-table-row/drops-the-trailing-blank-row: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`patched` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-table-row/drops-the-trailing-blank-row: committed diff did not carry before to after");
}

/// ⬆️ The LAST row is popped and the column count is untouched; the surviving row keeps its content, and the table stays above this leaf's 1-row floor.
#[semio_framework_async_macros::async_test]
async fn last_row_is_popped_and_the_one_row_floor_still_holds() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("remove-table-row applies");
    let NoteBlockNode::Table { rows: before_rows, .. } = find_block(&base.blocks, "blk-table").expect("the base table exists") else {
        panic!("remove-table-row/drops-the-trailing-blank-row: the base block must be a table");
    };
    assert_eq!(before_rows.len(), 2, "remove-table-row/drops-the-trailing-blank-row: the base table must start above the 1-row floor");
    let NoteBlockNode::Table { columns, rows, .. } = find_block(&applied.blocks, "blk-table").expect("the table survives") else {
        panic!("remove-table-row must not change the block's kind");
    };
    assert_eq!(rows.len(), 1, "remove-table-row/drops-the-trailing-blank-row: exactly one row must be popped");
    assert!(!rows.is_empty(), "this leaf refuses to go below its own 1-row floor");
    assert_eq!(columns.len(), 2, "removing a row must never remove a column");
    assert_eq!(rows[0][0].content, "Alpha", "the LAST row is the one popped — the first row's content survives");
}
