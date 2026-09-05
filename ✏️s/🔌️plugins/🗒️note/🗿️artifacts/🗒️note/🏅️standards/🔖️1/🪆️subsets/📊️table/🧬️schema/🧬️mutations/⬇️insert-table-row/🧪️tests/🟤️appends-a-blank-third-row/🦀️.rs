//! 🧪️ `insert-table-row` fixture — `🟤️appends-a-blank-third-row`.
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

/// ▶️ `insert-table-row` emits ONE whole-block `patched` entry appending a row whose width is read from the CURRENT column count.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let applied = apply_note_mutation(&before(), &mutation()).expect("insert-table-row applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "insert-table-row/appends-a-blank-third-row: applied state differs from committed after-snapshot");
}

/// ↩️ The inverse is `remove-table-row`, which pops the row just appended.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut snapshot = apply_note_mutation(&base, &forward).expect("insert-table-row applies forward");
    let mut undo = inverse_note_mutation(&base, &forward);
    undo.reverse();
    for step in &undo {
        snapshot = apply_note_mutation(&snapshot, step).expect("insert-table-row inverse step applies");
    }
    assert_eq!(snapshot, base, "insert-table-row/appends-a-blank-third-row: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: NoteSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-table-row/appends-a-blank-third-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "insert-table-row/appends-a-blank-third-row: committed mutation JSON is not canonical");
}

/// 🎯️ The block exists AND is a table, so the `mutation.target-missing` error guard fires for neither reason; this leaf has no no-op guard at all.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "insert-table-row/appends-a-blank-third-row: this fixture declares an applied outcome");
    let produced = mutation().diff(&before());
    let blocked = produced.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal));
    assert!(!blocked, "insert-table-row/appends-a-blank-third-row: declared applied but the diff builder rejected it: {:?}", produced.messages());
    apply_note_mutation(&before(), &mutation()).expect("insert-table-row/appends-a-blank-third-row: declared applied but the diff would not apply");
}

/// 🔺️ One `blocks.patched` entry whose `blockJson` holds three rows and still two columns — the appended row's width is baked into the committed delta.
///
/// The single most load-bearing assertion in the fixture: `before`+`after` only prove the end
/// state, whereas this pins WHICH collections and fields this mutation is allowed to touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-table-row/appends-a-blank-third-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff round-trips through the note artifact's own `NoteDiff`: its container is
/// `#[serde(default)]` with no `skip_serializing_if`, so all 23 fields must be present, `null` for
/// every slot `insert-table-row` leaves alone.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "insert-table-row/appends-a-blank-third-row: committed diff JSON is not canonical");
}

/// 🩹 The committed single-`patched` delta carries `before` to `after` on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: NoteDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <NoteDiff as protocol::MutationDiff<NoteSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-table-row/appends-a-blank-third-row: committed diff did not carry before to after");
}

/// ⬇️ A blank row is APPENDED at the bottom, sized to the current column count, leaving existing cell content alone.
#[semio_framework_async_macros::async_test]
async fn blank_row_is_appended_sized_to_the_current_column_count() {
    let base = before();
    let applied = apply_note_mutation(&base, &mutation()).expect("insert-table-row applies");
    let NoteBlockNode::Table { rows: before_rows, .. } = find_block(&base.blocks, "blk-table").expect("the base table exists") else {
        panic!("insert-table-row/appends-a-blank-third-row: the base block must be a table");
    };
    assert_eq!(before_rows.len(), 2, "insert-table-row/appends-a-blank-third-row: the base table must start with two rows");
    let NoteBlockNode::Table { columns, rows, .. } = find_block(&applied.blocks, "blk-table").expect("the table survives") else {
        panic!("insert-table-row must not change the block's kind");
    };
    assert_eq!(rows.len(), 3, "insert-table-row/appends-a-blank-third-row: exactly one row must be appended");
    assert_eq!(columns.len(), 2, "adding a row must never add a column");
    assert_eq!(rows[2].len(), columns.len(), "the appended row is sized from the CURRENT column count");
    assert!(rows[2].iter().all(|cell| cell.content.is_empty()), "the appended row must be blank");
    assert_eq!(rows[0][0].content, "Alpha", "appending must not disturb existing cell content");
}
