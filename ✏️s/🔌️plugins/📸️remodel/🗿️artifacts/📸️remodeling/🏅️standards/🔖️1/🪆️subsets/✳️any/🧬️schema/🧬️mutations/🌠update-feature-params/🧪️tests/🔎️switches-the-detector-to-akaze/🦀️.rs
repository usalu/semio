//! 🧪️ `update-feature-params` fixture — `🔎️switches-the-detector-to-akaze`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodeling::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
use crate::artifacts::remodeling::FeatureDetector;
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

/// ▶️ The detector enum and its budget move together — the payload is FINAL state for the whole
/// facet, so every field is written even where it repeats the base value.
#[semio_framework_async_macros::async_test]
async fn writes_the_akaze_detector_and_its_new_budget() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("update-feature-params applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-feature-params/switches-the-detector-to-akaze: applied state differs from committed after-snapshot");
    assert_eq!(applied.params.feature.detector, FeatureDetector::Akaze, "the detector switches away from the base ORB");
    assert_eq!(applied.params.feature.target_count, 8000, "the doubled keypoint budget is written");
    assert_eq!(applied.params.feature.octaves, 5, "the extra pyramid octave is written");
    assert_eq!(applied.params.matching, before().params.matching, "switching detectors never rewrites the matcher facet");
    assert_eq!(applied.results, before().results, "changing feature params never invalidates already-computed results");
}

/// ↩️ The inverse is the same verb carrying the captured base facet.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_base_orb_detector() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::UpdateFeatureParams(payload)] if payload.params.detector == FeatureDetector::Orb && payload.params.target_count == 4000),
        "update-feature-params inverts to itself with the base ORB facet, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-feature-params/switches-the-detector-to-akaze: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: a positive `target_count` and a finite non-negative `edge_threshold`
/// clear this leaf's FATAL `mutation.invariant`, which is checked BEFORE the no-op guard.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_positive_target_count_invariant() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-feature-params/switches-the-detector-to-akaze declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a positive target count with a finite edge threshold raises no mutation.invariant, got {:?}", produced.messages());
    let params = produced.diff().params.as_ref().expect("update-feature-params writes the params field");
    assert_eq!(params.feature.detector, FeatureDetector::Akaze, "the params delta carries the new feature facet");
    assert!(produced.diff().results.is_none(), "update-feature-params writes params alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-feature-params/switches-the-detector-to-akaze: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-feature-params/switches-the-detector-to-akaze: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-feature-params` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-feature-params/switches-the-detector-to-akaze: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let params = committed_diff.params.as_ref().expect("update-feature-params' delta is the whole params block");
    assert_eq!(params.feature.detector, FeatureDetector::Akaze, "the committed delta carries the AKAZE facet");
    assert_eq!(params.matching, before().params.matching, "and repeats all seven sibling facets unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-feature-params/switches-the-detector-to-akaze: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-feature-params`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-feature-params/switches-the-detector-to-akaze: committed diff did not carry before to after");
}
