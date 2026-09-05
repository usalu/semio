//! 🧪️ `change-run-language` fixture — `📅️retags-the-second-run-as-german`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an out-of-range BASE index is
//! `mutation.target-missing`, a tag equal to the run's current one is `mutation.no-op`, and
//! otherwise ONLY `runs[index].language` is assigned on a clone of `base.runs`. The before-snapshot
//! deliberately mis-tags a German word (`Welt`) as `en` so the retag is semantically real.

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
    serde_json::from_str(BEFORE).expect("change-run-language before snapshot decodes")
}
fn expected_after() -> SemioTextSnapshot {
    serde_json::from_str(AFTER).expect("change-run-language after snapshot decodes")
}
fn change_run_language() -> SemioTextMutation {
    serde_json::from_str(MUTATION).expect("change-run-language mutation decodes")
}

/// ▶️ Only the BCP-47 tag of run #1 flips from `en` to `de`; its body is untouched.
#[semio_framework_async_macros::async_test]
async fn retags_run_one_without_rewriting_its_body() {
    let base = before();
    let produced = change_run_language().diff(&base).diff().apply(&base).expect("change-run-language applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-run-language/retags-the-second-run-as-german: applied state differs from the committed after-snapshot");
    assert_eq!(produced.runs[1].language, "de", "run #1's BCP-47 tag must become the payload's new_language");
    assert_eq!(produced.runs[1].content, base.runs[1].content, "change-run-language must never rewrite the authored content");
    assert_eq!(produced.runs[0], base.runs[0], "the untargeted run #0 must keep its original `en` tag");
}

/// ↩️ The undo is another `change-run-language` carrying BASE's captured tag.
#[semio_framework_async_macros::async_test]
async fn the_undo_change_run_language_restores_the_english_tag() {
    let base = before();
    let mutation = change_run_language();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "change-run-language of an existing run undoes as exactly one change-run-language");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward change-run-language applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo change-run-language applies to the retagged state");
    }
    assert_eq!(current, base, "change-run-language/retags-the-second-run-as-german: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ChangeRunLanguage":{"index":1,"new_language":"de"}}` payload are
/// canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTextSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-run-language/retags-the-second-run-as-german: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(change_run_language()).expect("change-run-language mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-run-language mutation reparses");
    assert_eq!(reencoded, original, "change-run-language/retags-the-second-run-as-german: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: `de` differs from the run's current `en`, so `mutation.no-op` must not
/// fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_no_op_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-run-language/retags-the-second-run-as-german: this case is declared applied");
    let produced = change_run_language().diff(&before());
    assert!(produced.messages().is_empty(), "retagging to a genuinely different language must not raise mutation.no-op");
}

/// 🔺️ The produced whole-list delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(&change_run_language(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-run-language/retags-the-second-run-as-german: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and its second value already carries the German tag.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed change-run-language diff decodes");
    let list = decoded.runs.as_ref().expect("an applied change-run-language diff carries a runs list");
    assert_eq!(list.values[1].language, "de", "the diff itself must already carry the retagged run");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-run-language/retags-the-second-run-as-german: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed change-run-language diff decodes");
    let produced = decoded.apply(&before()).expect("committed change-run-language diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-run-language/retags-the-second-run-as-german: committed diff did not carry before to after");
}
