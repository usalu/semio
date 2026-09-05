//! 🧪️ `edit-run` fixture — `✍️rewrites-the-marked-runs-content`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: `edit-run` rejects an out-of-range BASE index
//! with `mutation.target-missing`, warns `mutation.no-op` when the content already equals
//! `new_content`, and otherwise assigns ONLY `runs[index].content` on a clone of `base.runs`.
//! Run #1 here carries a bold mark, which the edit must leave untouched — that is the point of
//! this case.

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
    serde_json::from_str(BEFORE).expect("edit-run before snapshot decodes")
}
fn expected_after() -> SemioTextSnapshot {
    serde_json::from_str(AFTER).expect("edit-run after snapshot decodes")
}
fn edit_run() -> SemioTextMutation {
    serde_json::from_str(MUTATION).expect("edit-run mutation decodes")
}

/// ▶️ Only `content` changes — the run's language tag and its bold mark are left alone.
#[semio_framework_async_macros::async_test]
async fn rewrites_only_the_content_of_run_one() {
    let base = before();
    let produced = edit_run().diff(&base).diff().apply(&base).expect("edit-run applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "edit-run/rewrites-the-marked-runs-content: applied state differs from the committed after-snapshot");
    assert_eq!(produced.runs.len(), base.runs.len(), "edit-run must never change how many runs there are");
    assert_eq!(produced.runs[1].content, "planet", "run #1's authored body must be replaced by new_content");
    assert_eq!(produced.runs[1].language, base.runs[1].language, "edit-run must not touch the run's BCP-47 tag");
    assert_eq!(produced.runs[1].marks, base.runs[1].marks, "edit-run must not touch the run's inline marks");
    assert_eq!(produced.runs[0], base.runs[0], "the untargeted run #0 must be byte-identical");
}

/// ↩️ `edit-run`'s undo is another `edit-run` restoring BASE's captured content.
#[semio_framework_async_macros::async_test]
async fn the_undo_edit_run_restores_the_original_body() {
    let base = before();
    let mutation = edit_run();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "edit-run of an existing run undoes as exactly one edit-run");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward edit-run applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo edit-run applies to the post-edit state");
    }
    assert_eq!(current, base, "edit-run/rewrites-the-marked-runs-content: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"EditRun":{"index":1,"new_content":"planet"}}` payload are canonical —
/// `EditRun` carries no `rename_all`, so the field stays snake_case `new_content` on the wire.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTextSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-run/rewrites-the-marked-runs-content: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(edit_run()).expect("edit-run mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("edit-run mutation reparses");
    assert_eq!(reencoded, original, "edit-run/rewrites-the-marked-runs-content: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: `"planet"` differs from the existing `"world"`, so neither the
/// `mutation.target-missing` rejection nor the `mutation.no-op` warning may fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_no_op_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-run/rewrites-the-marked-runs-content: this case is declared applied");
    let produced = edit_run().diff(&before());
    assert!(produced.messages().is_empty(), "editing a run to a genuinely different body must not raise mutation.no-op");
}

/// 🔺️ The produced whole-list delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(&edit_run(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-run/rewrites-the-marked-runs-content: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and still carries the bold mark on the rewritten run.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed edit-run diff decodes");
    let list = decoded.runs.as_ref().expect("an applied edit-run diff carries a runs list");
    assert_eq!(list.values[1].marks.len(), 1, "the rewritten run must keep its bold mark inside the diff itself");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-run/rewrites-the-marked-runs-content: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed edit-run diff decodes");
    let produced = decoded.apply(&before()).expect("committed edit-run diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-run/rewrites-the-marked-runs-content: committed diff did not carry before to after");
}
