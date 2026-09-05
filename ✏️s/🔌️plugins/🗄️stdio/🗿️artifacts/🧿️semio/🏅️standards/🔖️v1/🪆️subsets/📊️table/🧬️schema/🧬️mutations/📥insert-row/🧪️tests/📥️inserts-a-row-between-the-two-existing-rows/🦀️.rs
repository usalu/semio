//! 🧪️ `insert-row` fixture — `📥️inserts-a-row-between-the-two-existing-rows`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: the row lands at `min(index, rows.len())` and
//! an out-of-range index warns `mutation.clamped`. Crucially `columns` stays `None` — a row insert
//! never redeclares the schema — so the committed diff must carry `rows` and nothing else. The
//! inserted row's `cells` are authored pre-aligned with the existing two columns.

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
    serde_json::from_str(BEFORE).expect("insert-row before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("insert-row after snapshot decodes")
}
fn insert_row() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("insert-row mutation decodes")
}

/// ▶️ The Hamburg row lands between the two existing rows and is aligned with both columns.
#[semio_framework_async_macros::async_test]
async fn inserts_the_hamburg_row_at_final_index_one() {
    let base = before();
    let produced = insert_row().diff(&base).diff().apply(&base).expect("insert-row applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "insert-row/inserts-a-row-between-the-two-existing-rows: applied state differs from the committed after-snapshot");
    assert_eq!(produced.rows.len(), base.rows.len() + 1, "insert-row lengthens the row sequence by exactly one");
    assert_eq!(produced.rows[1].cells.len(), produced.columns.len(), "the inserted row must be positionally aligned with the column list");
    assert_eq!(produced.columns, base.columns, "insert-row must never redeclare the columns");
    assert_eq!((produced.rows[0].clone(), produced.rows[2].clone()), (base.rows[0].clone(), base.rows[1].clone()), "the two pre-existing rows survive unchanged, merely shifted");
}

/// ↩️ The undo is a single `remove-row` at the index the row landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_row_takes_the_hamburg_row_back_out() {
    let base = before();
    let mutation = insert_row();
    let undo = mutation.inverse(&base);
    assert_eq!(undo, vec![SemioTableMutation::RemoveRow(crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::remove_row::RemoveRow { index: 1 })], "insert-row at #1 must undo as remove-row at #1");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward insert-row applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo remove-row applies to the lengthened table");
    }
    assert_eq!(current, base, "insert-row/inserts-a-row-between-the-two-existing-rows: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"InsertRow":{"index":1,"row":{"cells":[…]}}}` payload are canonical —
/// `SemioValue` is internally tagged (`kind`), so an int cell encodes as `{"kind":"int","lexeme":…}`
/// with a STRING lexeme, never a JSON number.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-row/inserts-a-row-between-the-two-existing-rows: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(insert_row()).expect("insert-row mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("insert-row mutation reparses");
    assert_eq!(reencoded, original, "insert-row/inserts-a-row-between-the-two-existing-rows: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: index 1 is in range for a two-row base, so `mutation.clamped` must not
/// fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_clamp_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "insert-row/inserts-a-row-between-the-two-existing-rows: this case is declared applied");
    let produced = insert_row().diff(&before());
    assert!(produced.messages().is_empty(), "an in-range insert index must not raise mutation.clamped");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&insert_row(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-row/inserts-a-row-between-the-two-existing-rows: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical AND omits `columns` — a row insert may not restate the
/// schema.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_columns_entirely() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed insert-row diff decodes");
    assert!(decoded.columns.is_none(), "insert-row must leave the columns slot untouched");
    assert_eq!(decoded.rows.as_ref().map(|list| list.values.len()), Some(3), "the diff must carry all three rows of the final sequence");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("columns").is_none(), "the committed diff JSON must not carry a columns key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "insert-row/inserts-a-row-between-the-two-existing-rows: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed insert-row diff decodes");
    let produced = decoded.apply(&before()).expect("committed insert-row diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-row/inserts-a-row-between-the-two-existing-rows: committed diff did not carry before to after");
}
