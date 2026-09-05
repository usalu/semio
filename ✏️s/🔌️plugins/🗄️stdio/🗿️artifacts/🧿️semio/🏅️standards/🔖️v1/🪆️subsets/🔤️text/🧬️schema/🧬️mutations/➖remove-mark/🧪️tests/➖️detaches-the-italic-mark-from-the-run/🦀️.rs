//! 🧪️ `remove-mark` fixture — `➖️detaches-the-italic-mark-from-the-run`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: this leaf has TWO distinct
//! `mutation.target-missing` branches — an absent `run_index` (target `[run_index]`) and a mark
//! `index` past the end of that run's `marks` (target `[run_index, index]`). Neither fires here:
//! the run exists and carries two marks, and the italic one at nested index 1 is detached while
//! the bold one at index 0 stays.

use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::SemioTextDiff;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMarkKind, SemioTextSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioTextSnapshot {
    serde_json::from_str(BEFORE).expect("remove-mark before snapshot decodes")
}
fn expected_after() -> SemioTextSnapshot {
    serde_json::from_str(AFTER).expect("remove-mark after snapshot decodes")
}
fn remove_mark() -> SemioTextMutation {
    serde_json::from_str(MUTATION).expect("remove-mark mutation decodes")
}

/// ▶️ The italic mark at nested index 1 goes; the bold mark at index 0 stays.
#[semio_framework_async_macros::async_test]
async fn detaches_only_the_italic_mark() {
    let base = before();
    let produced = remove_mark().diff(&base).diff().apply(&base).expect("remove-mark applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "remove-mark/detaches-the-italic-mark-from-the-run: applied state differs from the committed after-snapshot");
    assert_eq!(produced.runs.len(), base.runs.len(), "remove-mark must never change the run sequence itself");
    assert_eq!(produced.runs[0].marks.len(), base.runs[0].marks.len() - 1, "the nested marks collection shrinks by exactly one");
    assert_eq!(produced.runs[0].marks[0].kind, SemioTextMarkKind::Bold, "the untargeted bold mark must remain at nested index 0");
    assert!(!produced.runs[0].marks.iter().any(|mark| mark.kind == SemioTextMarkKind::Italic), "the italic mark addressed by nested index 1 must be gone");
    assert_eq!(produced.runs[0].content, base.runs[0].content, "remove-mark must not touch the run's authored body");
}

/// ↩️ `remove-mark`'s undo re-attaches the captured mark at the same nested BASE index.
#[semio_framework_async_macros::async_test]
async fn the_undo_add_mark_reattaches_the_italic_mark_in_place() {
    let base = before();
    let mutation = remove_mark();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "remove-mark of an existing mark undoes as exactly one add-mark");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward remove-mark applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo add-mark applies to the unmarked state");
    }
    assert_eq!(current, base, "remove-mark/detaches-the-italic-mark-from-the-run: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RemoveMark":{"run_index":0,"index":1}}` payload are canonical fixed
/// points; flag-only marks encode their `href` as the empty string, never as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTextSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-mark/detaches-the-italic-mark-from-the-run: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(remove_mark()).expect("remove-mark mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("remove-mark mutation reparses");
    assert_eq!(reencoded, original, "remove-mark/detaches-the-italic-mark-from-the-run: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: neither the absent-run nor the out-of-range-mark rejection branch may
/// fire for run #0 / mark #1 of this base.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_neither_target_missing_branch_firing() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-mark/detaches-the-italic-mark-from-the-run: this case is declared applied");
    let produced = remove_mark().diff(&before());
    assert!(produced.messages().is_empty(), "an in-range run_index and mark index must raise neither mutation.target-missing branch");
}

/// 🔺️ The produced whole-list delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(&remove_mark(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-mark/detaches-the-italic-mark-from-the-run: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and carries the single surviving mark.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed remove-mark diff decodes");
    let list = decoded.runs.as_ref().expect("an applied remove-mark diff carries a runs list");
    assert_eq!(list.values[0].marks.len(), 1, "the diff must carry only the retained bold mark");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-mark/detaches-the-italic-mark-from-the-run: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed remove-mark diff decodes");
    let produced = decoded.apply(&before()).expect("committed remove-mark diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-mark/detaches-the-italic-mark-from-the-run: committed diff did not carry before to after");
}
