//! 🧪️ `replace-sparse` fixture — `swaps-in-an-uncolored-four-point-sparse-cloud`.
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

/// ▶️ `results.sparse` is replaced wholesale, dropping the base cloud's per-point colors: the
/// payload is an `Option<SparseCloud>`, never a per-buffer patch.
#[semio_framework_async_macros::async_test]
async fn replaces_the_cloud_and_drops_its_colors() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("replace-sparse applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud: applied state differs from committed after-snapshot");
    let sparse = applied.results.sparse.as_ref().expect("the sparse cloud is present");
    assert_eq!(sparse.points.to_f32_vec().len(), 12, "the new packed buffer decodes to four xyz triples");
    assert_eq!(sparse.colors, None, "a payload without colors CLEARS the base cloud's colors rather than keeping them");
    assert_eq!(applied.results.dense, before().results.dense, "the dense cloud is untouched");
    assert_eq!(applied.job.sparse_point_cloud_preview, before().job.sparse_point_cloud_preview, "the job's live preview buffer is a separate field and is not refreshed");
}

/// ↩️ The inverse is the same verb carrying the captured base cloud.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_colored_two_point_base_cloud() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::ReplaceSparse(payload)] if payload.sparse.as_ref().is_some_and(|sparse| sparse.colors.is_some())),
        "replace-sparse inverts to itself carrying the captured colored base cloud, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the payload differs from the base cloud, so the `mutation.no-op` warning —
/// this leaf's only guard — stays silent. Note there is no target-missing check: unlike
/// `replace-trajectory` and `replace-qc`, clearing an already-absent sparse cloud is not rejected.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_has_only_a_no_op_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a genuinely different sparse cloud raises no mutation.no-op, got {:?}", produced.messages());
    let results = produced.diff().results.as_ref().expect("replace-sparse writes the results field");
    assert!(results.sparse.is_some() && results.dense == before().results.dense, "the results delta carries the whole block with only sparse swapped");
    assert!(produced.diff().job.is_none(), "replace-sparse writes results alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-sparse` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let results = committed_diff.results.as_ref().expect("replace-sparse's delta is the whole results block");
    assert!(results.sparse.as_ref().is_some_and(|sparse| sparse.colors.is_none()), "the committed delta carries the uncolored replacement cloud");
    assert_eq!(results.tracks, before().results.tracks, "and repeats every results sibling unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-sparse`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-sparse/swaps-in-an-uncolored-four-point-sparse-cloud: committed diff did not carry before to after");
}
