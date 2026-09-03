//! 🧪️ `reorder-steps` fixture — `warns-that-an-over-clamped-index-leaves-the-tail-step-in-place`.
//!
//! `reorder-steps`'s diff oracle computes the target order on the ID LIST first — remove, then
//! `to_index.min(ids.len())`, then insert — and only mints a new `flow` handle when that list
//! actually differs. This case drives the clamp: `step-3` is already last, and index `9` clamps
//! back onto its own position, so the oracle short-circuits to a Warning `mutation.no-op` with an
//! empty diff. A no-op is APPLIED with nothing to apply, never a rejection.
//!
//! 🕸️ `flow` is a content-addressed CHILD handle whose `Path` lives in `imperative`'s thread-local
//! working-scene cache, so the committed `⬅️before` carries the handle and this file caches that
//! handle's program; together they ARE the before-state.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::{Dictionary, ProcedureSnapshot, Path, Step};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

/// 🛤️ The flat program the committed `flow` handle stands for: three `log.print` steps in order,
/// with `step-3` occupying the tail slot the payload's clamped index lands on.
fn cached_program() -> Path {
    Path { steps: ["step-1", "step-2", "step-3"].into_iter().map(|id| Step { id: id.into(), kind: "log.print".into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() }).collect() }
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
    serde_json::from_str(MUTATION).expect("reorder-steps mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<ProcedureDiff> {
    <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The clamped reorder is accepted and changes nothing: applying its diff reproduces the
/// committed after-document, whose `flow` handle is still the before-handle.
#[semio_framework_async_macros::async_test]
async fn the_clamped_reorder_carries_before_to_an_identical_after() {
    let base = before();
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("reorder-steps applies to its committed before-document");
    assert_eq!(applied, expected_after(), "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: the no-op reorder must reproduce the committed after-snapshot");
    assert_eq!(applied.flow, base.flow, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: a no-op reorder must not re-mint the content-addressed flow handle");
}

/// ↩️ `reorder-steps`'s inverse reads `id`'s CURRENT position out of BASE, so undoing this no-op is
/// a reorder of `step-3` back to index `2` — the exact slot it never left.
#[semio_framework_async_macros::async_test]
async fn the_inverse_reorders_the_tail_step_back_to_index_two() {
    let base = before();
    let inverse = <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: the inverse of one reorder is exactly one reorder back");
    let ProcedureMutation::ReorderSteps(undo) = &inverse[0] else {
        panic!("reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: reorder-steps' inverse must be a reorder-steps");
    };
    assert_eq!((undo.id.as_str(), undo.to_index), ("step-3", 2), "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: the undo must name step-3's real base position, not the payload's clamped 9");
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward reorder-steps applies");
    for step in &inverse {
        let redo = <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(redo.diff(), &snapshot).expect("the reorder-steps inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: undoing a no-op must still land back on the before-document");
}

/// 🔣️ Both committed documents and the `reorderSteps` payload are canonical — `toIndex` stays the
/// literal `9` the caller asked for; the clamp happens in the oracle, never in the payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProcedureSnapshot = serde_json::from_str(text).expect("imperative document decodes");
        let reencoded = serde_json::to_value(&decoded).expect("imperative document encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("imperative document reparses");
        assert_eq!(reencoded, original, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: committed {label} document JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("reorderSteps payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("reorderSteps payload reparses");
    assert_eq!(reencoded, original, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: committed reorderSteps JSON is not canonical");
}

/// 🎯️ A no-op is `applied` with a single Warning — `step-3` resolves, so this is never the Error
/// `mutation.target-missing` branch of the same oracle.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: a no-op is applied, not rejected");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), Some(protocol::Severity::Warning), "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: an unchanged order is a Warning, never an Error");
    assert_eq!(produced.messages().len(), 1, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: exactly one diagnostic is raised");
    assert_eq!(
        produced.messages()[0].code.0.as_str(),
        declared["messages"][0]["code"].as_str().expect("declared message code is a string"),
        "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: raised diagnostic code differs from the declared one"
    );
}

/// 🔺️ The committed diff is `ProcedureDiff`'s all-null default: the oracle returns before it ever
/// reaches `diff_replace_flow`, so `flow` — this mutation's ONLY output field — stays null, and so
/// do the presence/config lanes it must never touch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("produced reorder-steps diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `ProcedureDiff` and re-encodes unchanged — every one of the
/// seven fields is emitted as `null` because none carries `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: ProcedureDiff = serde_json::from_str(DIFF).expect("committed reorder-steps diff decodes");
    assert_eq!(decoded, ProcedureDiff::default(), "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: a no-op's committed diff must be the type's own default");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document — trivially, but
/// it must still be the committed diff that does it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: ProcedureDiff = serde_json::from_str(DIFF).expect("committed reorder-steps diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "reorder-steps/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place: committed diff did not carry before to after");
}
