//! 🧪️ `update-sfm-params` fixture — `🎯️switches-the-robust-loss-to-cauchy`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodeling::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
use crate::artifacts::remodeling::RobustLossKind;
use crate::artifacts::remodeling::{RemodelingDiff, RemodelingSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> RemodelingSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RemodelingSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RemodelingMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<RemodelingDiff> {
    <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The robust-loss kind and the Huber delta live in the same facet, so a Cauchy switch still
/// carries a `huber_delta_px` value — the schema keeps the slot regardless of the selected loss.
#[semio_framework_async_macros::async_test]
async fn writes_the_cauchy_loss_and_the_longer_ransac_budget() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("update-sfm-params applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-sfm-params/switches-the-robust-loss-to-cauchy: applied state differs from committed after-snapshot");
    assert_eq!(applied.params.sfm.robust_loss, RobustLossKind::Cauchy, "the robust loss switches away from the base Huber");
    assert_eq!(applied.params.sfm.ransac_iterations, 2000, "the doubled RANSAC budget is written");
    assert_eq!(applied.params.sfm.huber_delta_px, 0.75, "the Huber delta slot is still written even under a Cauchy loss");
    assert_eq!(applied.params.dense, before().params.dense, "the dense facet is carried through untouched");
    assert_eq!(applied.job, before().job, "changing SfM params never advances the reconstruction job");
}

/// ↩️ The inverse is the same verb carrying the captured base facet.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_base_huber_loss() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::UpdateSfmParams(payload)] if payload.params.robust_loss == RobustLossKind::Huber && payload.params.ransac_iterations == 1000),
        "update-sfm-params inverts to itself with the base Huber facet, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-sfm-params/switches-the-robust-loss-to-cauchy: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`. Unlike the ingest/feature/match leaves, this one checks the no-op FIRST
/// and only then its FATAL finite-threshold `mutation.invariant`.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_finite_threshold_invariant() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-sfm-params/switches-the-robust-loss-to-cauchy declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "finite RANSAC and Huber thresholds raise no mutation.invariant, got {:?}", produced.messages());
    let params = produced.diff().params.as_ref().expect("update-sfm-params writes the params field");
    assert_eq!(params.sfm.robust_loss, RobustLossKind::Cauchy, "the params delta carries the new SfM facet");
    assert!(produced.diff().job.is_none(), "update-sfm-params writes params alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-sfm-params/switches-the-robust-loss-to-cauchy: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-sfm-params/switches-the-robust-loss-to-cauchy: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-sfm-params` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-sfm-params/switches-the-robust-loss-to-cauchy: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let params = committed_diff.params.as_ref().expect("update-sfm-params' delta is the whole params block");
    assert_eq!(params.sfm.robust_loss, RobustLossKind::Cauchy, "the committed delta carries the Cauchy facet");
    assert_eq!(params.feature, before().params.feature, "and repeats all seven sibling facets unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-sfm-params/switches-the-robust-loss-to-cauchy: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-sfm-params`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-sfm-params/switches-the-robust-loss-to-cauchy: committed diff did not carry before to after");
}
