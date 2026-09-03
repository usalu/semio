//! 🧪️ `reorder-runs` fixture — `moves-the-first-run-to-the-end`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an out-of-range BASE `from` is
//! `mutation.target-missing`, `from == to` is `mutation.no-op`, and otherwise the run is REMOVED
//! first and then inserted at `min(to, len_after_removal)`. That remove-then-insert order is why
//! `from: 0, to: 2` over a three-run sequence lands the run last rather than second-to-last — the
//! whole point of this case.

use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::SemioTextDiff;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioTextSnapshot {
    serde_json::from_str(BEFORE).expect("reorder-runs before snapshot decodes")
}
fn expected_after() -> SemioTextSnapshot {
    serde_json::from_str(AFTER).expect("reorder-runs after snapshot decodes")
}
fn reorder_runs() -> SemioTextMutation {
    serde_json::from_str(MUTATION).expect("reorder-runs mutation decodes")
}

/// ▶️ `one` leaves the head and lands at the tail; `two`/`three` each shift one position down.
#[semio_framework_async_macros::async_test]
async fn moves_run_zero_past_the_other_two() {
    let base = before();
    let produced = reorder_runs().diff(&base).diff().apply(&base).expect("reorder-runs applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-runs/moves-the-first-run-to-the-end: applied state differs from the committed after-snapshot");
    assert_eq!(produced.runs.len(), base.runs.len(), "reorder-runs is a permutation — it may never add or drop a run");
    assert_eq!(produced.runs[2], base.runs[0], "the moved run must sit last after the remove-then-insert");
    assert_eq!((produced.runs[0].clone(), produced.runs[1].clone()), (base.runs[1].clone(), base.runs[2].clone()), "the two runs it jumped over must keep their relative order");
}

/// ↩️ The undo moves the run back from where it actually LANDED (`min(to, len - 1)`), not from the
/// requested `to`.
#[semio_framework_async_macros::async_test]
async fn the_undo_reorder_moves_the_run_back_to_the_head() {
    let base = before();
    let mutation = reorder_runs();
    let undo = mutation.inverse(&base);
    assert_eq!(
        undo,
        vec![SemioTextMutation::ReorderRuns(crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::reorder_runs::ReorderRuns { from: 2, to: 0 })],
        "the undo must address the landed index #2 and send it back to #0"
    );
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward reorder-runs applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo reorder-runs applies to the reordered state");
    }
    assert_eq!(current, base, "reorder-runs/moves-the-first-run-to-the-end: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ReorderRuns":{"from":0,"to":2}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTextSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-runs/moves-the-first-run-to-the-end: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(reorder_runs()).expect("reorder-runs mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("reorder-runs mutation reparses");
    assert_eq!(reencoded, original, "reorder-runs/moves-the-first-run-to-the-end: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: `from` (0) differs from `to` (2), so `mutation.no-op` must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_no_op_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-runs/moves-the-first-run-to-the-end: this case is declared applied");
    let produced = reorder_runs().diff(&before());
    assert!(produced.messages().is_empty(), "a genuine move must not raise mutation.no-op");
}

/// 🔺️ The produced whole-list delta equals the committed diff — the permuted sequence, nothing
/// else.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(&reorder_runs(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-runs/moves-the-first-run-to-the-end: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and is a strict permutation of the base sequence.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed reorder-runs diff decodes");
    let list = decoded.runs.as_ref().expect("an applied reorder-runs diff carries a runs list");
    assert_eq!(list.values.len(), before().runs.len(), "the reorder diff must carry exactly as many runs as the base");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-runs/moves-the-first-run-to-the-end: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed reorder-runs diff decodes");
    let produced = decoded.apply(&before()).expect("committed reorder-runs diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-runs/moves-the-first-run-to-the-end: committed diff did not carry before to after");
}
