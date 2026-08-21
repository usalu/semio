//! 🧪️ `remove-row` fixture — `removes-the-leading-row`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an out-of-range BASE index is Error
//! `mutation.target-missing`; otherwise `rows` is rebuilt without that entry and `columns` stays
//! `None`. Removing row #0 — the one that shifts every remaining index — is the case committed
//! here, and its inverse must re-insert at that same BASE index rather than appending.

use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::SemioTableDiff;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioTableSnapshot {
    serde_json::from_str(BEFORE).expect("remove-row before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("remove-row after snapshot decodes")
}
fn remove_row() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("remove-row mutation decodes")
}

/// ▶️ Row #0 goes; the Berlin row becomes the new head and the columns are untouched.
#[semio_framework_async_macros::async_test]
async fn removes_the_row_at_base_index_zero() {
    let base = before();
    let produced = remove_row().diff(&base).diff().apply(&base).expect("remove-row applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-row/removes-the-leading-row: applied state differs from the committed after-snapshot");
    assert_eq!(produced.rows.len(), base.rows.len() - 1, "remove-row shortens the row sequence by exactly one");
    assert_eq!(produced.rows[0], base.rows[1], "the row that followed the removed head becomes the new head");
    assert_eq!(produced.columns, base.columns, "remove-row must never redeclare the columns");
}

/// ↩️ The undo re-inserts the captured row at the same BASE index, restoring the original order.
#[semio_framework_async_macros::async_test]
async fn the_undo_insert_row_restores_the_head_row_in_place() {
    let base = before();
    let mutation = remove_row();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "remove-row of an existing row undoes as exactly one insert-row");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward remove-row applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo insert-row applies to the shortened table");
    }
    assert_eq!(current, base, "remove-row/removes-the-leading-row: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RemoveRow":{"index":0}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-row/removes-the-leading-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(remove_row()).expect("remove-row mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-row mutation reparses");
    assert_eq!(reencoded, original, "remove-row/removes-the-leading-row: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: row #0 exists, so `mutation.target-missing` must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_target_missing_rejection() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-row/removes-the-leading-row: this case is declared applied");
    let produced = remove_row().diff(&before());
    assert!(produced.messages().is_empty(), "an in-range remove index must not raise mutation.target-missing");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&remove_row(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-row/removes-the-leading-row: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical, carries the single surviving row, and omits `columns`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_columns_entirely() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed remove-row diff decodes");
    assert!(decoded.columns.is_none(), "remove-row must leave the columns slot untouched");
    assert_eq!(decoded.rows.as_ref().map(|list| list.values.len()), Some(1), "the diff must carry exactly the one surviving row");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-row/removes-the-leading-row: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed remove-row diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-row diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-row/removes-the-leading-row: committed diff did not carry before to after");
}
