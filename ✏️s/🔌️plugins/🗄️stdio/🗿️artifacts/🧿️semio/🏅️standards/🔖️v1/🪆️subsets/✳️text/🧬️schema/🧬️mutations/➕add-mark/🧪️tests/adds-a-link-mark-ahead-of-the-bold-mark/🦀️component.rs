//! 🧪️ `add-mark` fixture — `adds-a-link-mark-ahead-of-the-bold-mark`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an absent `run_index` is
//! `mutation.target-missing`, a mark already `contains`ed by the run is `mutation.no-op`, and
//! otherwise the mark is inserted into the NESTED `marks` collection at `min(index, marks.len())`.
//! `marks` is index-addressed one level deeper than `runs`, so this case pins insertion POSITION
//! inside the run, not mere membership.

use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::SemioTextDiff;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMarkKind, SemioTextSnapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioTextSnapshot {
    serde_json::from_str(BEFORE).expect("add-mark before snapshot decodes")
}
fn expected_after() -> SemioTextSnapshot {
    serde_json::from_str(AFTER).expect("add-mark after snapshot decodes")
}
fn add_mark() -> SemioTextMutation {
    serde_json::from_str(MUTATION).expect("add-mark mutation decodes")
}

/// ▶️ The link mark lands at nested index 0, pushing the pre-existing bold mark to index 1.
#[semio_framework_async_macros::async_test]
async fn inserts_the_link_mark_before_the_existing_bold_mark() {
    let base = before();
    let produced = add_mark().diff(&base).diff().apply(&base).expect("add-mark applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: applied state differs from the committed after-snapshot");
    assert_eq!(produced.runs.len(), base.runs.len(), "add-mark must never change the run sequence itself");
    assert_eq!(produced.runs[0].marks.len(), base.runs[0].marks.len() + 1, "the nested marks collection grows by exactly one");
    assert_eq!(produced.runs[0].marks[0].kind, SemioTextMarkKind::Link, "the new mark must occupy the FINAL-state index it was addressed with");
    assert_eq!(produced.runs[0].marks[0].href, "https://semio.tech", "a Link mark is the only kind that carries a non-empty href");
    assert_eq!(produced.runs[0].marks[1], base.runs[0].marks[0], "the pre-existing bold mark must survive, merely shifted");
    assert_eq!(produced.runs[0].content, base.runs[0].content, "add-mark must not touch the run's authored body");
}

/// ↩️ `add-mark`'s undo is a `remove-mark` at the clamped index the mark landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_mark_detaches_the_link_again() {
    let base = before();
    let mutation = add_mark();
    let undo = mutation.inverse(&base);
    assert_eq!(
        undo,
        vec![SemioTextMutation::RemoveMark(crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::remove_mark::mutation::RemoveMark { run_index: 0, index: 0 })],
        "add-mark at run #0/#0 must undo as remove-mark at run #0/#0"
    );
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward add-mark applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo remove-mark applies to the marked state");
    }
    assert_eq!(current, base, "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"AddMark":{"run_index":0,"index":0,"mark":{…}}}` payload are canonical —
/// `SemioTextMarkKind` is `rename_all = "camelCase"`, so `Link` is `"link"` on the wire.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTextSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(add_mark()).expect("add-mark mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("add-mark mutation reparses");
    assert_eq!(reencoded, original, "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the run does not already `contains` this link mark, so
/// `mutation.no-op` must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_no_op_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: this case is declared applied");
    let produced = add_mark().diff(&before());
    assert!(produced.messages().is_empty(), "attaching a mark the run does not already carry must not raise mutation.no-op");
}

/// 🔺️ The produced whole-list delta equals the committed diff — a nested `marks` edit still
/// travels as the rebuilt top-level `runs` list, and nothing beyond it.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(&add_mark(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and already carries both marks in their final order.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed add-mark diff decodes");
    let list = decoded.runs.as_ref().expect("an applied add-mark diff carries a runs list");
    assert_eq!(list.values[0].marks.len(), 2, "the diff must carry the link mark alongside the retained bold mark");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed add-mark diff decodes");
    let produced = decoded.apply(&before()).expect("committed add-mark diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-mark/adds-a-link-mark-ahead-of-the-bold-mark: committed diff did not carry before to after");
}
