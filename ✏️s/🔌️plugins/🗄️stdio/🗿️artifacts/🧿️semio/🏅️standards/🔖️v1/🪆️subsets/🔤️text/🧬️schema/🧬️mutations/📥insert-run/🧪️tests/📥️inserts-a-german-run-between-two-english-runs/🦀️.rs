//! 🧪️ `insert-run` fixture — `📥️inserts-a-german-run-between-two-english-runs`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation here was transcribed from
//! `../../🔺️diff/🦀️.rs`: `insert-run` clones `base.runs`, inserts the payload run at
//! `min(index, len)` and wraps the WHOLE rebuilt sequence in `SemioTextRunList` — so the diff is
//! `runs.values = the complete three-run sequence`, never a sparse positional delta.
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from this quintet by `fixtures generate`, never hand-forged here.

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
    serde_json::from_str(BEFORE).expect("insert-run before snapshot decodes")
}
fn expected_after() -> SemioTextSnapshot {
    serde_json::from_str(AFTER).expect("insert-run after snapshot decodes")
}
fn insert_run() -> SemioTextMutation {
    serde_json::from_str(MUTATION).expect("insert-run mutation decodes")
}

/// ▶️ The German run lands at FINAL index 1, between the two English runs, and nothing else moves.
#[semio_framework_async_macros::async_test]
async fn inserts_the_german_run_at_final_index_one() {
    let base = before();
    let produced = insert_run().diff(&base).diff().apply(&base).expect("insert-run applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "insert-run/inserts-a-german-run-between-two-english-runs: applied state differs from the committed after-snapshot");
    assert_eq!(produced.runs.len(), base.runs.len() + 1, "insert-run must lengthen the run sequence by exactly one");
    assert_eq!(produced.runs[1].language, "de", "the inserted run keeps its own BCP-47 tag at the FINAL-state index it was addressed with");
    assert_eq!((produced.runs[0].clone(), produced.runs[2].clone()), (base.runs[0].clone(), base.runs[1].clone()), "the two pre-existing English runs must survive unchanged, merely shifted");
}

/// ↩️ `insert-run`'s undo is a single `remove-run` at the index the run landed at.
#[semio_framework_async_macros::async_test]
async fn the_undo_remove_run_takes_the_german_run_back_out() {
    let base = before();
    let mutation = insert_run();
    let undo = mutation.inverse(&base);
    assert_eq!(undo, vec![SemioTextMutation::RemoveRun(crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::remove_run::RemoveRun { index: 1 })], "insert-run at #1 must undo as remove-run at #1");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward insert-run applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo remove-run applies to the post-insert state");
    }
    assert_eq!(current, base, "insert-run/inserts-a-german-run-between-two-english-runs: the undo did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the externally-tagged `{"InsertRun":{…}}` payload are already
/// canonical: decode→encode is a fixed point (`InsertRun` carries no `rename_all`, so its fields
/// stay `index`/`run` verbatim).
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioTextSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-run/inserts-a-german-run-between-two-english-runs: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(insert_run()).expect("insert-run mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("insert-run mutation reparses");
    assert_eq!(reencoded, original, "insert-run/inserts-a-german-run-between-two-english-runs: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied` with no diagnostics — index 1 is in range for a two-run base, so the
/// diff leaf's `mutation.clamped` warning branch must NOT fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_clamp_warning() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "insert-run/inserts-a-german-run-between-two-english-runs: this case is declared applied");
    let produced = insert_run().diff(&before());
    assert!(produced.messages().is_empty(), "an in-range insert index must not raise mutation.clamped");
}

/// 🔺️ The whole-list delta this insert produces is exactly the committed diff — it pins that
/// `insert-run` touches `runs` and only `runs`, rebuilt positionally from `base`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(&insert_run(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-run/inserts-a-german-run-between-two-english-runs: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `SemioTextDiff` — `runs` is the type's only
/// field, and it is `skip_serializing_if = "Option::is_none"`, so a populated `runs` key is the
/// entire encoding.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed insert-run diff decodes");
    assert!(decoded.runs.is_some(), "an applied insert-run diff must carry a runs list, not an empty diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "insert-run/inserts-a-german-run-between-two-english-runs: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the run list in the diff
/// is a complete description of the insert, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioTextDiff = serde_json::from_str(DIFF).expect("committed insert-run diff decodes");
    let produced = decoded.apply(&before()).expect("committed insert-run diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-run/inserts-a-german-run-between-two-english-runs: committed diff did not carry before to after");
}
