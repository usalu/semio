//! 🧪️ `replace-dense` fixture — `swaps-in-a-two-point-classified-dense-cloud`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodel::mutations::{apply_remodel_mutation, inverse_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::{RemodelDiff, RemodelSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> RemodelSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RemodelSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RemodelMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<RemodelDiff> {
    <RemodelMutation as protocol::Mutation<RemodelSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ All four packed buffers of `results.dense` — positions, colors, confidence and LAS-style
/// classification codes — are replaced together as one `Option<DenseCloud>`.
#[semio_framework_async_macros::async_test]
async fn replaces_all_four_dense_buffers_together() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("replace-dense applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-dense/swaps-in-a-two-point-classified-dense-cloud: applied state differs from committed after-snapshot");
    let dense = applied.results.dense.as_ref().expect("the dense cloud is present");
    assert_eq!(dense.positions.to_f32_vec().len(), 6, "the new positions buffer decodes to two xyz triples");
    assert_eq!(dense.classification.as_ref().expect("classification codes are present").to_u8_vec(), vec![2, 6], "the LAS ground/building codes are stored as raw bytes");
    assert_eq!(dense.confidence.as_ref().expect("confidence is present").to_f32_vec(), vec![0.75, 0.5], "the per-point confidence buffer is replaced alongside the positions");
    assert_eq!(applied.results.sparse, before().results.sparse, "the sparse cloud is untouched");
    assert_eq!(applied.params.dense, before().params.dense, "replacing the dense result never rewrites the dense params that produced it");
}

/// ↩️ The inverse is the same verb carrying the captured base cloud.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_single_point_base_cloud() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::ReplaceDense(payload)] if payload.dense.as_ref().is_some_and(|dense| dense.positions.to_f32_vec().len() == 3)),
        "replace-dense inverts to itself carrying the captured single-point base cloud, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-dense/swaps-in-a-two-point-classified-dense-cloud: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the payload differs from the base cloud, so the `mutation.no-op` warning —
/// this leaf's only guard — stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_has_only_a_no_op_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-dense/swaps-in-a-two-point-classified-dense-cloud declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a genuinely different dense cloud raises no mutation.no-op, got {:?}", produced.messages());
    let results = produced.diff().results.as_ref().expect("replace-dense writes the results field");
    assert_eq!(results.mesh, before().results.mesh, "the results delta carries the mesh sibling unchanged");
    assert!(produced.diff().params.is_none(), "replace-dense writes results alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-dense/swaps-in-a-two-point-classified-dense-cloud: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-dense/swaps-in-a-two-point-classified-dense-cloud: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-dense` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-dense/swaps-in-a-two-point-classified-dense-cloud: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let results = committed_diff.results.as_ref().expect("replace-dense's delta is the whole results block");
    assert!(results.dense.as_ref().is_some_and(|dense| dense.classification.is_some()), "the committed delta carries the classified replacement cloud");
    assert_eq!(results.sparse, before().results.sparse, "and repeats every results sibling unchanged — the delta is block-level, not field-level");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-dense/swaps-in-a-two-point-classified-dense-cloud: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-dense`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-dense/swaps-in-a-two-point-classified-dense-cloud: committed diff did not carry before to after");
}
