//! 🧪️ `create-column` fixture — `appends-a-float-column-and-null-pads-every-row`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: a duplicate `name` is FATAL
//! `mutation.duplicate-id`; otherwise the column lands at
//! `at = index.unwrap_or(len).min(len)` AND a `SemioValue::Null` is inserted at that same `at`
//! into every row's `cells`. That second half is the CRITICAL row/column alignment invariant, so
//! this case's diff must carry BOTH `columns` and `rows` — a diff that only rebuilt `columns`
//! would leave every row one cell short.

use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::SemioTableDiff;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioTableSnapshot {
    serde_json::from_str(BEFORE).expect("create-column before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("create-column after snapshot decodes")
}
fn create_column() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("create-column mutation decodes")
}

/// ▶️ The `area` column appears at index 1 and every row gains a `Null` cell at the same index.
#[semio_framework_async_macros::async_test]
async fn creates_the_area_column_and_pads_each_row_with_null() {
    let base = before();
    let produced = create_column().diff(&base).diff().apply(&base).expect("create-column applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-column/appends-a-float-column-and-null-pads-every-row: applied state differs from the committed after-snapshot");
    assert_eq!(produced.columns.len(), base.columns.len() + 1, "create-column adds exactly one column");
    assert_eq!(produced.columns[1].name, "area", "the new column must land at the requested FINAL index");
    assert_eq!(produced.columns[1].kind, SemioTableCellKind::Float, "the declared column kind travels with the payload");
    assert_eq!(produced.rows.len(), base.rows.len(), "create-column must not add or drop rows");
    for (index, row) in produced.rows.iter().enumerate() {
        assert_eq!(row.cells.len(), base.rows[index].cells.len() + 1, "row #{index} must stay positionally aligned with the widened column list");
        assert_eq!(row.cells[1], SemioValue::Null, "the padding cell for a brand-new column is always SemioValue::Null");
    }
}

/// ↩️ `create-column`'s undo is a single `delete-column` by name, whose own cascade handling takes
/// the padding cells back out.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_column_removes_the_column_and_its_padding() {
    let base = before();
    let mutation = create_column();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-column undoes as exactly one delete-column");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-column applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-column applies to the widened table");
    }
    assert_eq!(current, base, "create-column/appends-a-float-column-and-null-pads-every-row: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"CreateColumn":{"name":…,"kind":"float","index":1}}` payload are
/// canonical — `index` is a bare `Option<usize>` with no `skip_serializing_if`, so it is always
/// present on the wire (here as a real number, never omitted).
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-column/appends-a-float-column-and-null-pads-every-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(create_column()).expect("create-column mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-column mutation reparses");
    assert_eq!(reencoded, original, "create-column/appends-a-float-column-and-null-pads-every-row: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no column named `area` exists in the base, so the FATAL
/// `mutation.duplicate-id` branch must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_duplicate_id_rejection() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-column/appends-a-float-column-and-null-pads-every-row: this case is declared applied");
    let produced = create_column().diff(&before());
    assert!(produced.messages().is_empty(), "creating a column with a fresh name must raise no diagnostics at all");
}

/// 🔺️ The produced delta equals the committed diff — BOTH slots populated, which is what pins the
/// alignment cascade.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&create_column(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-column/appends-a-float-column-and-null-pads-every-row: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and populates both `columns` and `rows`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_carries_both_slots() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed create-column diff decodes");
    assert!(decoded.columns.is_some(), "create-column must rebuild the column list");
    assert!(decoded.rows.is_some(), "create-column must ALSO rebuild the row list — the Null padding is part of the same diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-column/appends-a-float-column-and-null-pads-every-row: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed create-column diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-column diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-column/appends-a-float-column-and-null-pads-every-row: committed diff did not carry before to after");
}
