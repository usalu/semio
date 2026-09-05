//! 🧪️ `delete-column` fixture — `🗑️drops-the-middle-column-and-cascades-into-every-row`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown name is Error
//! `mutation.target-missing`; otherwise the column at its BASE position is removed AND the cell at
//! that same position is removed from every row whose `cells` reach that far, with an INFO
//! `mutation.cascade` message counting the rows actually touched. Both rows are touched here, so
//! the declared outcome carries that message.

use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::SemioTableDiff;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioTableSnapshot {
    serde_json::from_str(BEFORE).expect("delete-column before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("delete-column after snapshot decodes")
}
fn delete_column() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("delete-column mutation decodes")
}

/// ▶️ `city` and every row's cell #1 disappear together; the surviving cells stay aligned.
#[semio_framework_async_macros::async_test]
async fn deletes_the_city_column_and_its_cell_in_every_row() {
    let base = before();
    let produced = delete_column().diff(&base).diff().apply(&base).expect("delete-column applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-column/drops-the-middle-column-and-cascades-into-every-row: applied state differs from the committed after-snapshot");
    assert!(!produced.columns.iter().any(|column| column.name == "city"), "the named column must be gone");
    assert_eq!(produced.rows.len(), base.rows.len(), "delete-column must not drop whole rows");
    for (index, row) in produced.rows.iter().enumerate() {
        assert_eq!(row.cells.len(), produced.columns.len(), "row #{index} must remain positionally aligned with the narrowed column list");
        assert_eq!(row.cells[1], base.rows[index].cells[2], "the cell that followed the deleted column must slide into its place");
    }
}

/// ↩️ The undo re-creates the column at its BASE index and then replays one `edit-cell` per row —
/// a bare re-create would only refill `Null`.
#[semio_framework_async_macros::async_test]
async fn the_undo_recreates_the_column_and_restores_every_captured_cell() {
    let base = before();
    let mutation = delete_column();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1 + base.rows.len(), "the undo is one create-column plus one edit-cell per row that carried a cell");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-column applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "delete-column/drops-the-middle-column-and-cascades-into-every-row: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteColumn":{"name":"city"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-column/drops-the-middle-column-and-cascades-into-every-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(delete_column()).expect("delete-column mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-column mutation reparses");
    assert_eq!(reencoded, original, "delete-column/drops-the-middle-column-and-cascades-into-every-row: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` WITH the `mutation.cascade` note — an INFO message is a diagnostic, not
/// a rejection, and both rows really were touched.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_including_the_cascade_note() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-column/drops-the-middle-column-and-cascades-into-every-row: this case is declared applied");
    let produced = delete_column().diff(&before());
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "deleting a column that really cascaded raises exactly one message");
    assert_eq!(messages[0].code.0, "mutation.cascade", "the message must be the cascade note, not a rejection");
    assert_eq!(messages[0].level, protocol::Severity::Info, "a cascade note is INFO — the mutation still applies");
}

/// 🔺️ The produced delta equals the committed diff — both slots, because the cascade is part of
/// the same diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&delete_column(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-column/drops-the-middle-column-and-cascades-into-every-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and its rebuilt rows are already narrowed.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed delete-column diff decodes");
    let rows = decoded.rows.as_ref().expect("delete-column must rebuild the row list, not only the column list");
    assert!(rows.values.iter().all(|row| row.cells.len() == 2), "every rebuilt row in the diff must already carry only two cells");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-column/drops-the-middle-column-and-cascades-into-every-row: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed delete-column diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-column diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-column/drops-the-middle-column-and-cascades-into-every-row: committed diff did not carry before to after");
}
