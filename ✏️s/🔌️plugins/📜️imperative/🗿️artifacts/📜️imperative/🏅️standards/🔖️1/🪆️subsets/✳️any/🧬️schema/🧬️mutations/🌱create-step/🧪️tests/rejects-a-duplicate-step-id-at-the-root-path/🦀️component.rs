//! 🧪️ `create-step` fixture — `rejects-a-duplicate-step-id-at-the-root-path`.
//!
//! `create-step`'s diff oracle guards the target list twice: Fatal `mutation.invariant` when
//! `pathRef.owner` names a container that does not exist, and Fatal `mutation.duplicate-id` when
//! the payload's own `step.id` is already taken in the addressed list. This case pins the second
//! guard at the ROOT path, where the container check is skipped entirely.
//!
//! 🕸️ The `flow` field is a content-addressed CHILD handle, and `imperative`'s working scene is a
//! thread-local scratch cache (`ImperativeWorkingScene`): a persisted handle only round-trips to
//! real `Path` content within a process that cached it. So the committed `⬅️before` carries the
//! handle and this file caches that handle's `Path` — the two together ARE the before-state.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, Path, PathRef, Step};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const ABSENT_DIFF: &str = include_str!("🔺️diff/🚫️component.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

/// 🛤️ The root program the committed `flow` handle stands for: `step-1` then `step-2`, both
/// `log.print`, no nested bodies — so `step-1` is unambiguously taken.
fn cached_program() -> Path {
    Path {
        steps: vec![
            Step { id: "step-1".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() },
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
    serde_json::from_str(MUTATION).expect("create-step mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<ImperativeDiff> {
    <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(&mutation(), &before())
}

/// ⛔️ A duplicate `step-1` never reaches the program: the document — including its `flow` handle,
/// which a successful create would have re-minted — is byte-identical afterwards.
#[semio_framework_async_macros::async_test]
async fn the_rejected_create_leaves_the_flow_handle_untouched() {
    let base = before();
    let carried = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("an empty diff always applies");
    assert_eq!(carried, expected_after(), "create-step/rejects-a-duplicate-step-id-at-the-root-path: the rejected create must leave the committed after-snapshot equal to before");
    assert_eq!(carried.flow, base.flow, "create-step/rejects-a-duplicate-step-id-at-the-root-path: a rejected create must not re-mint the content-addressed flow handle");
}

/// ↩️ `create-step`'s inverse is payload-derived, not base-derived: it hands back a `delete-step`
/// of the id it was asked to mint regardless of the outcome. Nothing undoes a fatal create — the
/// dispatcher must never run that step — so this pins the shape without applying it.
#[semio_framework_async_macros::async_test]
async fn the_unused_inverse_is_a_delete_of_the_contested_id() {
    let base = before();
    let inverse = <ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "create-step/rejects-a-duplicate-step-id-at-the-root-path: create-step always proposes exactly one undo step");
    let ImperativeMutation::DeleteStep(undo) = &inverse[0] else {
        panic!("create-step/rejects-a-duplicate-step-id-at-the-root-path: create-step's inverse must be a delete-step");
    };
    assert_eq!((undo.id.as_str(), &undo.path_ref), ("step-1", &PathRef::default()), "create-step/rejects-a-duplicate-step-id-at-the-root-path: the undo must address the contested id at the root path");
    assert_eq!(before(), expected_after(), "create-step/rejects-a-duplicate-step-id-at-the-root-path: with the forward step rejected there is nothing to restore");
}

/// 🔣️ Both committed documents and the `createStep` payload are canonical — note `pathRef`
/// serializes to the bare `{}` because both of its fields carry `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ImperativeSnapshot = serde_json::from_str(text).expect("imperative document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("imperative document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("imperative document reparses");
        assert_eq!(reencoded, original, "create-step/rejects-a-duplicate-step-id-at-the-root-path: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("createStep payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("createStep payload reparses");
    assert_eq!(reencoded, original, "create-step/rejects-a-duplicate-step-id-at-the-root-path: committed createStep JSON is not canonical");
}

/// 🎯️ The declared rejection is exactly the one `create-step` raises: `Fatal`,
/// `mutation.duplicate-id`, addressed at `step-1` — never the container-level `mutation.invariant`,
/// which this root-path fixture deliberately cannot reach.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-step/rejects-a-duplicate-step-id-at-the-root-path: this fixture declares a rejected outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), Some(protocol::Severity::Fatal), "create-step/rejects-a-duplicate-step-id-at-the-root-path: a taken step id is Fatal, not merely an Error");
    assert_eq!(produced.messages().len(), 1, "create-step/rejects-a-duplicate-step-id-at-the-root-path: exactly one fault is raised");
    assert_eq!(produced.messages()[0].code.0.as_str(), declared["code"].as_str().expect("declared code is a string"), "create-step/rejects-a-duplicate-step-id-at-the-root-path: raised fault code differs from the declared one");
    assert_eq!(produced.messages()[0].target, vec!["step-1".to_string()], "create-step/rejects-a-duplicate-step-id-at-the-root-path: the fault must address the contested step id");
}

/// 🔺️ MutationOutcome LAW 1: a `Fatal` message forces `diff == D::default()`. For `create-step`
/// that means no `flow` replacement handle is minted at all.
#[semio_framework_async_macros::async_test]
async fn produces_no_diff() {
    let produced = built_outcome();
    assert_eq!(produced.diff(), &ImperativeDiff::default(), "create-step/rejects-a-duplicate-step-id-at-the-root-path: a fatal create must produce the default diff");
    assert!(produced.diff().flow.is_none(), "create-step/rejects-a-duplicate-step-id-at-the-root-path: no replacement flow handle may be minted for a rejected create");
}

/// 🚫️ The case carries the empty `🔺️diff/🚫️component.absent` marker instead of an invented patch —
/// there is no serialized `create-step` delta to commit for a rejection.
#[semio_framework_async_macros::async_test]
async fn committed_absent_marker_is_empty() {
    assert!(ABSENT_DIFF.is_empty(), "create-step/rejects-a-duplicate-step-id-at-the-root-path: 🚫️component.absent must stay empty, never a stand-in diff");
}

/// 🩹 With no diff to apply, `before` IS `after`: the committed pair must be identical documents.
#[semio_framework_async_macros::async_test]
async fn before_and_after_are_the_same_document() {
    assert_eq!(before(), expected_after(), "create-step/rejects-a-duplicate-step-id-at-the-root-path: a rejected case's after-snapshot must equal its before-snapshot");
}
