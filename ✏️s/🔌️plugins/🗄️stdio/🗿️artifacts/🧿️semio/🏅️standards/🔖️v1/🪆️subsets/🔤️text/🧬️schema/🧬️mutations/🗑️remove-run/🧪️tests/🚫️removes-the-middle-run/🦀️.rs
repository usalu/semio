//! 🧪️ `remove-run` fixture — `🚫️removes-the-middle-run`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: `remove-run` rejects an out-of-range BASE
//! index with `mutation.target-missing` and otherwise clones `base.runs`, calls `Vec::remove` at
//! the BASE index, and wraps the whole shortened sequence in `SemioTextRunList`. The German run at
//! #1 is the one taken out here; the two English runs close up around it.

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
    serde_json::from_str(BEFORE).expect("remove-run before snapshot decodes")
}
fn expected_after() -> SemioTextSnapshot {
    serde_json::from_str(AFTER).expect("remove-run after snapshot decodes")
}
fn remove_run() -> SemioTextMutation {
    serde_json::from_str(MUTATION).expect("remove-run mutation decodes")
}

/// ▶️ Run #1 (`de`/`Beta`) disappears and the surrounding English runs close up.
#[semio_framework_async_macros::async_test]
async fn removes_the_german_run_at_base_index_one() {
    let base = before();
    let produced = remove_run().diff(&base).diff().apply(&base).expect("remove-run applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-run/removes-the-middle-run: applied state differs from the committed after-snapshot");
    assert_eq!(produced.runs.len(), base.runs.len() - 1, "remove-run must shorten the run sequence by exactly one");
    assert!(!produced.runs.iter().any(|run| run.language == "de"), "the German run addressed by BASE index #1 must be gone");
    assert_eq!(produced.runs[1], base.runs[2], "the run that followed the removed one must slide down into its place");
}

/// ↩️ `remove-run`'s undo re-inserts the captured run at the same BASE index.
#[semio_framework_async_macros::async_test]
async fn the_undo_insert_run_puts_the_german_run_back_in_the_middle() {
    let base = before();
    let mutation = remove_run();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "remove-run of an existing run undoes as exactly one insert-run");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward remove-run applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo insert-run applies to the post-remove state");
    }
    assert_eq!(current, base, "remove-run/removes-the-middle-run: the undo did not restore the before-snapshot");
}

/// 🔣️ Both snapshots and the `{"RemoveRun":{"index":1}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTextSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-run/removes-the-middle-run: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(remove_run()).expect("remove-run mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-run mutation reparses");
    assert_eq!(reencoded, original, "remove-run/removes-the-middle-run: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: index 1 exists in a three-run base, so the `mutation.target-missing`
/// rejection branch must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_target_missing_rejection() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-run/removes-the-middle-run: this case is declared applied");
    let produced = remove_run().diff(&before());
    assert!(produced.messages().is_empty(), "an in-range remove index must not raise mutation.target-missing");
}

/// 🔺️ The produced whole-list delta equals the committed diff — `remove-run` may rewrite `runs`
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(&remove_run(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-run/removes-the-middle-run: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical, and its `runs.values` is the SHORTENED sequence.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed remove-run diff decodes");
    assert_eq!(decoded.runs.as_ref().map(|list| list.values.len()), Some(2), "the committed remove-run diff must carry the two surviving runs");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-run/removes-the-middle-run: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed remove-run diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-run diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-run/removes-the-middle-run: committed diff did not carry before to after");
}
