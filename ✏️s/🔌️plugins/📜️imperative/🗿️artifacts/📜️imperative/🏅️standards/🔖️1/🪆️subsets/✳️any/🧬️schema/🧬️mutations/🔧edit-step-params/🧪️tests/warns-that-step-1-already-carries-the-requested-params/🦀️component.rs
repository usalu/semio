//! 🧪️ `edit-step-params` fixture — `warns-that-step-1-already-carries-the-requested-params`.
//!
//! `edit-step-params` replaces a step's `params` dictionary WHOLESALE, and its diff oracle compares
//! the requested replacement against the step's current dictionary before doing anything. This case
//! drives that equality guard: `step-1` already carries exactly `{ message: "Guten Tag" }`, so the
//! oracle short-circuits to a Warning `mutation.no-op` with an empty diff — applied, with nothing
//! to apply, and no `flow` handle re-minted.
//!
//! 🕸️ `flow` is a content-addressed CHILD handle whose `Path` lives in `imperative`'s thread-local
//! working-scene cache, so the committed `⬅️before` carries the handle and this file caches that
//! handle's program; together they ARE the before-state.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, Path, Step};
use neural_engine::{Atom, Value};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

/// 🛤️ The program the committed `flow` handle stands for: `step-1` already prints "Guten Tag",
/// while `step-2` carries the empty dictionary that makes the guard's target unambiguous.
fn cached_program() -> Path {
    Path {
        steps: vec![
            Step {
                id: "step-1".into(),
                kind: "log.print".into(),
                params: Dictionary::new().insert("message", Value::Atom(Atom::String("Guten Tag".into()))),
                bodies: std::collections::BTreeMap::new(),
            },
            Step { id: "step-2".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() },
        ],
    }
}

fn before() -> ImperativeSnapshot {
    let snapshot: ImperativeSnapshot = serde_json::from_str(BEFORE).expect("before imperative document decodes");
    crate::artifacts::imperative::cache_imperative_flow(&snapshot.flow.child_id, &cached_program());
    snapshot
}
fn expected_after() -> ImperativeSnapshot {
    serde_json::from_str(AFTER).expect("after imperative document decodes")
}
fn mutation() -> ImperativeMutation {
    serde_json::from_str(MUTATION).expect("edit-step-params mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<ImperativeDiff> {
    <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The idempotent param edit is accepted and changes nothing: applying its diff reproduces the
/// committed after-document, whose `flow` handle is still the before-handle.
#[semio_framework_async_macros::async_test]
async fn the_idempotent_edit_carries_before_to_an_identical_after() {
    let base = before();
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("edit-step-params applies to its committed before-document");
    assert_eq!(applied, expected_after(), "edit-step-params/warns-that-step-1-already-carries-the-requested-params: the no-op edit must reproduce the committed after-snapshot");
    assert_eq!(applied.flow, base.flow, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: a no-op edit must not re-mint the content-addressed flow handle");
}

/// ↩️ `edit-step-params`'s inverse re-sends the OLD dictionary read out of BASE — which here is
/// byte-equal to the payload's own `newParams`, the very reason the forward edit was a no-op.
#[semio_framework_async_macros::async_test]
async fn the_inverse_resends_the_identical_dictionary() {
    let base = before();
    let inverse = <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: the inverse of one param edit is exactly one param edit back");
    let ImperativeMutation::EditStepParams(undo) = &inverse[0] else {
        panic!("edit-step-params/warns-that-step-1-already-carries-the-requested-params: edit-step-params' inverse must be an edit-step-params");
    };
    assert_eq!(undo.new_params, cached_program().steps[0].params, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: the undo must carry step-1's own base dictionary");
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward edit-step-params applies");
    for step in &inverse {
        let redo = <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(redo.diff(), &snapshot).expect("the edit-step-params inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: undoing a no-op must still land back on the before-document");
}

/// 🔣️ Both committed documents and the `editStepParams` payload are canonical — `newParams` is a
/// `#[serde(transparent)]` `Dictionary`, so it rides as a bare JSON object, not a wrapper.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ImperativeSnapshot = serde_json::from_str(text).expect("imperative document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("imperative document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("imperative document reparses");
        assert_eq!(reencoded, original, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("editStepParams payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("editStepParams payload reparses");
    assert_eq!(reencoded, original, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: committed editStepParams JSON is not canonical");
}

/// 🎯️ An already-identical dictionary is `applied` with a single Warning — `step-1` resolves, so
/// this is never the Error `mutation.target-missing` branch of the same oracle.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-step-params/warns-that-step-1-already-carries-the-requested-params: a no-op is applied, not rejected");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), Some(protocol::Severity::Warning), "edit-step-params/warns-that-step-1-already-carries-the-requested-params: an unchanged dictionary is a Warning, never an Error");
    assert_eq!(produced.messages().len(), 1, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: exactly one diagnostic is raised");
    assert_eq!(produced.messages()[0].code.0.as_str(), declared["messages"][0]["code"].as_str().expect("declared message code is a string"), "edit-step-params/warns-that-step-1-already-carries-the-requested-params: raised diagnostic code differs from the declared one");
}

/// 🔺️ The committed diff is `ImperativeDiff`'s all-null default: the oracle returns before it ever
/// reaches `diff_replace_flow`, so `flow` — this mutation's ONLY output field — stays null, and the
/// sibling `text` seed handle is never dragged along either.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced edit-step-params diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `ImperativeDiff` and re-encodes unchanged — every one of the
/// seven fields is emitted as `null` because none carries `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ImperativeDiff = serde_json::from_str(DIFF).expect("committed edit-step-params diff decodes");
    assert_eq!(decoded, ImperativeDiff::default(), "edit-step-params/warns-that-step-1-already-carries-the-requested-params: a no-op's committed diff must be the type's own default");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-step-params/warns-that-step-1-already-carries-the-requested-params: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document — trivially, but
/// it must still be the committed diff that does it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ImperativeDiff = serde_json::from_str(DIFF).expect("committed edit-step-params diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "edit-step-params/warns-that-step-1-already-carries-the-requested-params: committed diff did not carry before to after");
}
