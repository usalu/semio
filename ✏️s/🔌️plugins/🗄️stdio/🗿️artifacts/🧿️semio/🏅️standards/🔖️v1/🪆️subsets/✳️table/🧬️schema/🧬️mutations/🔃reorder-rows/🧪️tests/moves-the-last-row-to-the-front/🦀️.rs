//! 🧪️ `reorder-rows` fixture — `moves-the-last-row-to-the-front`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an out-of-range BASE `from` is Error
//! `mutation.target-missing`, `from == to` is Warning `mutation.no-op`, and otherwise the row is
//! REMOVED first and re-inserted at `min(to, len_after_removal)`. `columns` stays `None`: rows are
//! an anonymous ordered collection, so a permutation of them cannot affect the schema.

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
    serde_json::from_str(BEFORE).expect("reorder-rows before snapshot decodes")
}
fn expected_after() -> SemioTableSnapshot {
    serde_json::from_str(AFTER).expect("reorder-rows after snapshot decodes")
}
fn reorder_rows() -> SemioTableMutation {
    serde_json::from_str(MUTATION).expect("reorder-rows mutation decodes")
}

/// ▶️ The Hamburg row leaves the tail and becomes the head; the other two shift down one.
#[semio_framework_async_macros::async_test]
async fn moves_row_two_to_the_head_of_the_sequence() {
    let base = before();
    let produced = reorder_rows().diff(&base).diff().apply(&base).expect("reorder-rows applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-rows/moves-the-last-row-to-the-front: applied state differs from the committed after-snapshot");
    assert_eq!(produced.rows.len(), base.rows.len(), "reorder-rows is a permutation — it may never add or drop a row");
    assert_eq!(produced.rows[0], base.rows[2], "the moved row must sit first afterwards");
    assert_eq!((produced.rows[1].clone(), produced.rows[2].clone()), (base.rows[0].clone(), base.rows[1].clone()), "the rows it jumped over keep their relative order");
    assert_eq!(produced.columns, base.columns, "a row permutation must never redeclare the columns");
}

/// ↩️ The undo addresses the index the row LANDED at (`min(to, len - 1)` = 0) and sends it back
/// to 2.
#[semio_framework_async_macros::async_test]
async fn the_undo_reorder_sends_the_row_back_to_the_tail() {
    let base = before();
    let mutation = reorder_rows();
    let undo = mutation.inverse(&base);
    assert_eq!(
        undo,
        vec![SemioTableMutation::ReorderRows(crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::reorder_rows::ReorderRows { from: 0, to: 2 })],
        "the undo must address the landed index #0 and send it back to #2"
    );
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward reorder-rows applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo reorder-rows applies to the reordered table");
    }
    assert_eq!(current, base, "reorder-rows/moves-the-last-row-to-the-front: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ReorderRows":{"from":2,"to":0}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTableSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-rows/moves-the-last-row-to-the-front: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(reorder_rows()).expect("reorder-rows mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("reorder-rows mutation reparses");
    assert_eq!(reencoded, original, "reorder-rows/moves-the-last-row-to-the-front: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: `from` (2) differs from `to` (0), so `mutation.no-op` must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_no_op_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-rows/moves-the-last-row-to-the-front: this case is declared applied");
    let produced = reorder_rows().diff(&before());
    assert!(produced.messages().is_empty(), "a genuine row move must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(&reorder_rows(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-rows/moves-the-last-row-to-the-front: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical, is a strict permutation, and omits `columns`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_omits_columns_entirely() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed reorder-rows diff decodes");
    assert!(decoded.columns.is_none(), "reorder-rows must leave the columns slot untouched");
    assert_eq!(decoded.rows.as_ref().map(|list| list.values.len()), Some(before().rows.len()), "the reorder diff must carry exactly as many rows as the base");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-rows/moves-the-last-row-to-the-front: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTableDiff = serde_json::from_str(DIFF).expect("committed reorder-rows diff decodes");
    let produced = decoded.apply(&before()).expect("committed reorder-rows diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-rows/moves-the-last-row-to-the-front: committed diff did not carry before to after");
}
