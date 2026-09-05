//! 🧪️ `edit-cell` fixture — `⚫️rewrites-the-population-cell-of-the-second-row`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, which has THREE Error `mutation.target-missing`
//! branches — unknown `column_name`, out-of-range `row_index`, and a row too short to reach the
//! resolved column index — plus a Warning `mutation.no-op` when the cell already holds
//! `new_value`. None fire here. The cell is addressed by `{row_index, column_name}` but WRITTEN at
//! the column's resolved POSITION, so this case pins that name→index resolution.

use crate::artifacts::semio::standards::v1::subsets::table::schema::diff::SemioTableDiff;
use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioTableSnapshot {
    serde_json::from_str(BEFORE).expect("edit-cell before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("edit-cell after snapshot decodes")
}
fn edit_cell() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("edit-cell mutation decodes")
}

/// ▶️ Exactly one cell — row #1, resolved column index 1 — takes the new value.
#[semio_framework_async_macros::async_test]
async fn rewrites_only_the_addressed_cell() {
    let base = before();
    let produced = edit_cell().diff(&base).diff().apply(&base).expect("edit-cell applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "edit-cell/rewrites-the-population-cell-of-the-second-row: applied state differs from the committed after-snapshot");
    assert_eq!(produced.rows[1].cells[1], SemioValue::Int { lexeme: "3755251".to_string() }, "the addressed cell must hold new_value");
    assert_eq!(produced.rows[1].cells[0], base.rows[1].cells[0], "the sibling cell in the same row must be untouched");
    assert_eq!(produced.rows[0], base.rows[0], "the untargeted row must be byte-identical");
    assert_eq!(produced.columns, base.columns, "edit-cell must never redeclare the columns");
}

/// ↩️ The undo is another `edit-cell` carrying BASE's captured value at the same address.
#[semio_framework_async_macros::async_test]
async fn the_undo_edit_cell_restores_the_captured_value() {
    let base = before();
    let mutation = edit_cell();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "edit-cell of a reachable cell undoes as exactly one edit-cell");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward edit-cell applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo edit-cell applies to the edited table");
    }
    assert_eq!(current, base, "edit-cell/rewrites-the-population-cell-of-the-second-row: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"EditCell":{"row_index":1,"column_name":"population","new_value":{…}}}`
/// payload are canonical — `new_value` is a full internally-tagged `SemioValue`, not a bare scalar.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-cell/rewrites-the-population-cell-of-the-second-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(edit_cell()).expect("edit-cell mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("edit-cell mutation reparses");
    assert_eq!(reencoded, original, "edit-cell/rewrites-the-population-cell-of-the-second-row: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the column resolves, the row exists and is long enough, and the new
/// value differs from the current one — so none of the four guard branches may fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_guard_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-cell/rewrites-the-population-cell-of-the-second-row: this case is declared applied");
    let produced = edit_cell().diff(&before());
    assert!(produced.messages().is_empty(), "a reachable cell edited to a genuinely different value must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&edit_cell(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-cell/rewrites-the-population-cell-of-the-second-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical, omits `columns`, and already carries the new cell value.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_columns_entirely() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed edit-cell diff decodes");
    assert!(decoded.columns.is_none(), "edit-cell must leave the columns slot untouched");
    let rows = decoded.rows.as_ref().expect("an applied edit-cell diff carries a rows list");
    assert_eq!(rows.values[1].cells[1], SemioValue::Int { lexeme: "3755251".to_string() }, "the diff itself must already carry the rewritten cell");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-cell/rewrites-the-population-cell-of-the-second-row: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed edit-cell diff decodes");
    let produced = decoded.apply(&before()).expect("committed edit-cell diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-cell/rewrites-the-population-cell-of-the-second-row: committed diff did not carry before to after");
}
