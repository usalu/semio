//! 🧪️ `update-ingest-params` fixture — `tightens-the-ingest-sharpness-gate`.
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

/// ▶️ Only the `ingest` facet of `ReconstructionParams` is rewritten; the other seven facets are
/// carried through untouched even though the diff replaces the whole params block.
#[semio_framework_async_macros::async_test]
async fn rewrites_the_ingest_facet_alone() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("update-ingest-params applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-ingest-params/tightens-the-ingest-sharpness-gate: applied state differs from committed after-snapshot");
    assert_eq!(applied.params.ingest.min_sharpness, 0.5, "the tightened blur gate is written");
    assert_eq!(applied.params.ingest.frame_sample_stride, 2, "the denser sampling stride is written");
    assert_eq!(applied.params.feature, before().params.feature, "the feature facet is carried through untouched");
    assert_eq!(applied.params.geo, before().params.geo, "the geo facet is carried through untouched");
    assert_eq!(applied.streams, before().streams, "changing ingest params never re-samples already-imported frames");
}

/// ↩️ The inverse is the same verb carrying the captured base facet.
#[semio_framework_async_macros::async_test]
async fn inverse_is_the_same_verb_carrying_the_base_ingest_facet() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelMutation::UpdateIngestParams(payload)] if payload.params.min_sharpness == 0.25 && payload.params.max_frames == 200), "update-ingest-params inverts to itself with the base facet, got {inverse:?}");
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-ingest-params/tightens-the-ingest-sharpness-gate: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`. This leaf checks its FATAL invariant FIRST: a finite non-negative
/// `min_sharpness` plus non-zero `max_frames` and `frame_sample_stride`; only then the no-op.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_positive_sampling_invariant() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-ingest-params/tightens-the-ingest-sharpness-gate declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a finite sharpness with non-zero max-frames and stride raises no mutation.invariant, got {:?}", produced.messages());
    let params = produced.diff().params.as_ref().expect("update-ingest-params writes the params field");
    assert_eq!(params.ingest.max_frames, 400, "the params delta carries the new ingest facet");
    assert_eq!(params.mesh, before().params.mesh, "the params delta carries every sibling facet unchanged");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-ingest-params/tightens-the-ingest-sharpness-gate: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-ingest-params/tightens-the-ingest-sharpness-gate: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-ingest-params` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-ingest-params/tightens-the-ingest-sharpness-gate: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let params = committed_diff.params.as_ref().expect("update-ingest-params' delta is the whole params block");
    assert_eq!(params.ingest.min_sharpness, 0.5, "the committed delta carries the tightened blur gate");
    assert_eq!(params.geo, before().params.geo, "and repeats all seven sibling facets unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-ingest-params/tightens-the-ingest-sharpness-gate: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-ingest-params`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-ingest-params/tightens-the-ingest-sharpness-gate: committed diff did not carry before to after");
}
