//! 🧪️ `delete-step` fixture — `rejects-a-root-step-id-addressed-inside-a-branch-body`.
//!
//! `delete-step`'s diff oracle resolves its target through `resolve_steps(base, path_ref)` — the
//! step list the `PathRef` ADDRESSES, never the whole program. This case pins that: `step-1` really
//! is in the document, but it lives at the root, and the payload addresses `step-3`'s `then` body,
//! so the delete is an Error `mutation.target-missing` rather than a cross-scope removal.
//!
//! 🕸️ `flow` is a content-addressed CHILD handle whose `Path` lives in `imperative`'s thread-local
//! working-scene cache, so the committed `⬅️before` carries the handle and this file caches that
//! handle's nested program; together they ARE the before-state.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::{Dictionary, ProcedureSnapshot, Path, Step};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const ABSENT_DIFF: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

/// 🛤️ The nested program the committed `flow` handle stands for: `step-1` at the root, and a
/// `control.if` step whose `then` body holds only `step-3a`.
fn cached_program() -> Path {
    let mut bodies = std::collections::BTreeMap::new();
    bodies.insert("then".to_string(), Path { steps: vec![Step { id: "step-3a".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() }] });
    Path { steps: vec![Step { id: "step-1".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() }, Step { id: "step-3".into(), kind: "control.if".into(), params: Dictionary::new(), bodies }] }
}

fn before() -> ProcedureSnapshot {
    let mut snapshot: ProcedureSnapshot = serde_json::from_str(BEFORE).expect("before imperative document decodes");
    crate::artifacts::procedure::materialize_procedure_flow(&mut snapshot.flow, &cached_program());
    snapshot
}
fn expected_after() -> ProcedureSnapshot {
    serde_json::from_str(AFTER).expect("after imperative document decodes")
}
fn mutation() -> ProcedureMutation {
    serde_json::from_str(MUTATION).expect("delete-step mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<ProcedureDiff> {
    <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::diff(&mutation(), &before())
}

/// ⛔️ The out-of-scope delete removes nothing: neither `step-1` at the root nor `step-3a` inside
/// the branch is disturbed, and the `flow` handle is not re-minted.
#[semio_framework_async_macros::async_test]
async fn the_rejected_delete_leaves_both_scopes_intact() {
    let base = before();
    let carried = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("an empty diff always applies");
    assert_eq!(carried, expected_after(), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: the rejected delete must leave the committed after-snapshot equal to before");
    let scene = crate::artifacts::procedure::procedure_working_scene(&carried);
    assert!(scene.path.steps.iter().any(|step| step.id == "step-1"), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: the root step named by the payload must survive an out-of-scope delete");
}

/// ↩️ `delete-step`'s inverse is BASE-derived: it re-creates the step it found. With no step found
/// at the addressed body slot there is nothing to re-create, so the inverse is empty.
#[semio_framework_async_macros::async_test]
async fn the_inverse_of_a_missed_target_is_empty() {
    let base = before();
    let inverse = <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::inverse(&mutation(), &base);
    assert!(inverse.is_empty(), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: a delete that resolved no target must propose no undo step");
    assert_eq!(before(), expected_after(), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: with the forward step rejected there is nothing to restore");
}

/// 🔣️ Both committed documents and the `deleteStep` payload are canonical — here `pathRef` really
/// does serialize both of its optional fields, unlike a root-addressed payload's bare `{}`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProcedureSnapshot = serde_json::from_str(text).expect("imperative document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("imperative document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("imperative document reparses");
        assert_eq!(reencoded, original, "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("deleteStep payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("deleteStep payload reparses");
    assert_eq!(reencoded, original, "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: committed deleteStep JSON is not canonical");
}

/// 🎯️ `delete-step` grades an unresolvable target as `Error`, not `Fatal` — the addressed list is
/// simply already without that id — and addresses the fault at the id it could not find.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: this fixture declares a rejected outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), Some(protocol::Severity::Error), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: a missing delete target is an Error, never a Fatal");
    assert_eq!(produced.messages().len(), 1, "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: exactly one fault is raised");
    assert_eq!(produced.messages()[0].code.0.as_str(), declared["code"].as_str().expect("declared code is a string"), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: raised fault code differs from the declared one");
    assert_eq!(produced.messages()[0].target, vec!["step-1".to_string()], "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: the fault must address the id the payload asked to delete");
}

/// 🔺️ MutationOutcome LAW 2: an `Error` message means the diff carries no change for the named
/// target — for `delete-step` that is the whole diff, since `flow` is its only output field.
#[semio_framework_async_macros::async_test]
async fn produces_no_diff() {
    let produced = built_outcome();
    assert_eq!(produced.diff(), &ProcedureDiff::default(), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: a target-missing delete must produce the default diff");
    assert!(produced.diff().flow.is_none(), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: no replacement flow handle may be minted when nothing was removed");
}

/// 🚫️ The case carries the empty `🔺️diff/🚫️.absent` marker instead of an invented patch.
#[semio_framework_async_macros::async_test]
async fn committed_absent_marker_is_empty() {
    assert!(ABSENT_DIFF.is_empty(), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: 🚫️.absent must stay empty, never a stand-in diff");
}

/// 🩹 With no diff to apply, `before` IS `after`: the committed pair must be identical documents.
#[semio_framework_async_macros::async_test]
async fn before_and_after_are_the_same_document() {
    assert_eq!(before(), expected_after(), "delete-step/rejects-a-root-step-id-addressed-inside-a-branch-body: a rejected case's after-snapshot must equal its before-snapshot");
}
