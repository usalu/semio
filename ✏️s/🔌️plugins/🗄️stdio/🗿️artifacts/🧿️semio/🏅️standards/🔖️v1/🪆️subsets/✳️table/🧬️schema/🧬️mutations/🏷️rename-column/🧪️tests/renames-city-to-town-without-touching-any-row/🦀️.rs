//! 🧪️ `rename-column` fixture — `renames-city-to-town-without-touching-any-row`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, which has THREE guard branches — Error
//! `mutation.target-missing` (old name absent), Warning `mutation.no-op` (`name == new_name`),
//! FATAL `mutation.duplicate-id` (`new_name` already taken) — and then rebuilds ONLY `columns`,
//! leaving `rows: None`. That `None` is the whole point of this case: a rename is a pure
//! identity-field change, so the committed diff must not even mention `rows`.

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
    serde_json::from_str(BEFORE).expect("rename-column before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("rename-column after snapshot decodes")
}
fn rename_column() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("rename-column mutation decodes")
}

/// ▶️ Only the column's native key changes — its declared kind and every row stay put.
#[semio_framework_async_macros::async_test]
async fn renames_the_column_key_and_leaves_the_rows_alone() {
    let base = before();
    let produced = rename_column().diff(&base).diff().apply(&base).expect("rename-column applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "rename-column/renames-city-to-town-without-touching-any-row: applied state differs from the committed after-snapshot");
    assert_eq!(produced.columns[0].name, "town", "the column's native key must become new_name");
    assert_eq!(produced.columns[0].kind, base.columns[0].kind, "a rename must never change the declared cell kind");
    assert_eq!(produced.rows, base.rows, "a rename touches no row — cells are positional, not name-keyed");
}

/// ↩️ The undo swaps `name`/`new_name`, looking the OLD name up in `base`.
#[semio_framework_async_macros::async_test]
async fn the_undo_rename_column_swaps_the_two_names_back() {
    let base = before();
    let mutation = rename_column();
    let undo = mutation.inverse(&base);
    assert_eq!(
        undo,
        vec![SemioTableMutation::RenameColumn(crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::rename_column::mutation::RenameColumn { name: "town".to_string(), new_name: "city".to_string() })],
        "the undo must address the NEW name and rename it back to the old one"
    );
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward rename-column applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo rename-column applies to the renamed table");
    }
    assert_eq!(current, base, "rename-column/renames-city-to-town-without-touching-any-row: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RenameColumn":{"name":"city","new_name":"town"}}` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-column/renames-city-to-town-without-touching-any-row: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(rename_column()).expect("rename-column mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rename-column mutation reparses");
    assert_eq!(reencoded, original, "rename-column/renames-city-to-town-without-touching-any-row: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: `town` is neither the old name nor an existing one, so neither the
/// `mutation.no-op` warning nor the `mutation.duplicate-id` FATAL may fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_no_no_op_and_no_duplicate_id() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-column/renames-city-to-town-without-touching-any-row: this case is declared applied");
    let produced = rename_column().diff(&before());
    assert!(produced.messages().is_empty(), "renaming to a genuinely free name must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff — `columns` only.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&rename_column(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-column/renames-city-to-town-without-touching-any-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical AND leaves `rows` unset — `skip_serializing_if` means the
/// key is absent from the JSON entirely, which is what proves the rename never reached the rows.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_rows_entirely() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed rename-column diff decodes");
    assert!(decoded.columns.is_some(), "rename-column must rebuild the column list");
    assert!(decoded.rows.is_none(), "rename-column must leave the rows slot untouched");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert!(committed.get("rows").is_none(), "the committed diff JSON must not carry a rows key at all");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "rename-column/renames-city-to-town-without-touching-any-row: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed rename-column diff decodes");
    let produced = decoded.apply(&before()).expect("committed rename-column diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-column/renames-city-to-town-without-touching-any-row: committed diff did not carry before to after");
}
