//! 🧪️ `update-motion-params` fixture — `enables-motion-tracking`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodeling::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation, RemodelingMutation};
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

/// ▶️ Flipping `enabled` on is a pure params change: no track is created, and the one motion
/// track already in `results` is neither re-classified nor re-measured.
#[semio_framework_async_macros::async_test]
async fn enables_tracking_without_touching_the_existing_track() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("update-motion-params applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-motion-params/enables-motion-tracking: applied state differs from committed after-snapshot");
    assert!(applied.params.motion.enabled, "motion tracking is switched on");
    assert_eq!(applied.params.motion.max_tracks, 128, "the doubled track budget is written");
    assert_eq!(applied.params.motion.min_track_quality, 0.5, "the raised quality gate is written");
    assert_eq!(applied.results.tracks, before().results.tracks, "enabling tracking never fabricates or re-scores tracks");
    assert_eq!(applied.params.geo, before().params.geo, "the geo facet is carried through untouched");
}

/// ↩️ The inverse is the same verb carrying the captured base facet.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_disabled_base_facet() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelingMutation::UpdateMotionParams(payload)] if !payload.params.enabled && payload.params.max_tracks == 64), "update-motion-params inverts to itself with the base disabled facet, got {inverse:?}");
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-motion-params/enables-motion-tracking: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the no-op guard runs first, then the FATAL finite-`min_track_quality`
/// `mutation.invariant` — the only number this leaf validates.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_finite_track_quality_invariant() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-motion-params/enables-motion-tracking declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a finite minimum track quality raises no mutation.invariant, got {:?}", produced.messages());
    let params = produced.diff().params.as_ref().expect("update-motion-params writes the params field");
    assert!(params.motion.enabled, "the params delta carries the new motion facet");
    assert!(produced.diff().results.is_none(), "update-motion-params writes params alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-motion-params/enables-motion-tracking: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-motion-params/enables-motion-tracking: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-motion-params` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-motion-params/enables-motion-tracking: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let params = committed_diff.params.as_ref().expect("update-motion-params' delta is the whole params block");
    assert!(params.motion.enabled, "the committed delta carries the enabled motion facet");
    assert_eq!(params.dense, before().params.dense, "and repeats all seven sibling facets unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-motion-params/enables-motion-tracking: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-motion-params`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-motion-params/enables-motion-tracking: committed diff did not carry before to after");
}
