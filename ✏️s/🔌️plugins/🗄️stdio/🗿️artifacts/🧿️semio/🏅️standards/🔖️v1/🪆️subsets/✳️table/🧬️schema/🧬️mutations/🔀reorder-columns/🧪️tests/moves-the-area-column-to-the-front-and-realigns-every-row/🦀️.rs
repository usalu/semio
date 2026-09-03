//! 🧪️ `reorder-columns` fixture — `moves-the-area-column-to-the-front-and-realigns-every-row`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown name is Error
//! `mutation.target-missing`, `from == to_index` is Warning `mutation.no-op`, and otherwise the
//! IDENTICAL remove-then-insert (`from` → `min(to_index, len-1)`) is replayed on every row's
//! `cells`. Moving the LAST column to the front is the case that would break loudest if the row
//! half of that cascade were missing, which is why it is the one committed here.

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
    serde_json::from_str(BEFORE).expect("reorder-columns before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("reorder-columns after snapshot decodes")
}
fn reorder_columns() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("reorder-columns mutation decodes")
}

/// ▶️ `area` jumps from last to first, and every row's third cell makes the identical jump.
#[semio_framework_async_macros::async_test]
async fn moves_the_area_column_and_replays_the_move_on_every_row() {
    let base = before();
    let produced = reorder_columns().diff(&base).diff().apply(&base).expect("reorder-columns applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: applied state differs from the committed after-snapshot");
    assert_eq!(produced.columns.len(), base.columns.len(), "reorder-columns is a permutation — it may never add or drop a column");
    assert_eq!(produced.columns[0].name, "area", "the addressed column must sit at to_index afterwards");
    for (index, row) in produced.rows.iter().enumerate() {
        assert_eq!(row.cells[0], base.rows[index].cells[2], "row #{index}'s cell must follow its column to the front");
        assert_eq!(row.cells[1], base.rows[index].cells[0], "the cells the moved column jumped over keep their relative order");
        assert_eq!(row.cells[2], base.rows[index].cells[1], "the cells the moved column jumped over keep their relative order");
    }
}

/// ↩️ The undo sends the column back to its ORIGINAL BASE index (2), not to some clamped position.
#[semio_framework_async_macros::async_test]
async fn the_undo_reorder_returns_the_column_to_its_base_index() {
    let base = before();
    let mutation = reorder_columns();
    let undo = mutation.inverse(&base);
    assert_eq!(
        undo,
        vec![SemioTableMutation::ReorderColumns(crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::reorder_columns::ReorderColumns { name: "area".to_string(), to_index: 2 })],
        "the undo must send the column back to the index it originally occupied in base"
    );
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward reorder-columns applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo reorder-columns applies to the reordered table");
    }
    assert_eq!(current, base, "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ReorderColumns":{"name":"area","to_index":0}}` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(reorder_columns()).expect("reorder-columns mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("reorder-columns mutation reparses");
    assert_eq!(reencoded, original, "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the column's BASE index (2) differs from `to_index` (0), so
/// `mutation.no-op` must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_no_op_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: this case is declared applied");
    let produced = reorder_columns().diff(&before());
    assert!(produced.messages().is_empty(), "a genuine column move must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff — both slots, because the row realignment is
/// part of the same diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&reorder_columns(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and carries the permuted columns AND the permuted cells.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_carries_both_slots() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed reorder-columns diff decodes");
    assert_eq!(decoded.columns.as_ref().map(|list| list.values[0].name.clone()), Some("area".to_string()), "the diff's own column list must already be permuted");
    assert!(decoded.rows.is_some(), "reorder-columns must ALSO rebuild the row list — the cell realignment is part of the same diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed reorder-columns diff decodes");
    let produced = decoded.apply(&before()).expect("committed reorder-columns diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-columns/moves-the-area-column-to-the-front-and-realigns-every-row: committed diff did not carry before to after");
}
