//! 🧪️ `update-geo-params` fixture — `🟨️enables-georeferencing-with-an-origin`.
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

/// ▶️ The three optional origin components go from `None` to real coordinates in the same write
/// that enables georeferencing; the derived geo products in `results` are not re-projected.
#[semio_framework_async_macros::async_test]
async fn fills_in_the_origin_while_enabling_georeferencing() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("update-geo-params applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-geo-params/enables-georeferencing-with-an-origin: applied state differs from committed after-snapshot");
    assert!(applied.params.geo.enabled, "georeferencing is switched on");
    assert_eq!(applied.params.geo.origin_lat, Some(52.375), "the surveyed origin latitude is written");
    assert_eq!(applied.params.geo.origin_lon, Some(8.5), "the surveyed origin longitude is written");
    assert_eq!(applied.params.geo.gsd_m, 0.03125, "the finer ground sampling distance is written");
    assert_eq!(applied.results.geo, before().results.geo, "the derived DSM/DTM/ortho references are not re-projected by a params change");
}

/// ↩️ The inverse is the same verb carrying the captured base facet, restoring the null origin.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_null_origin() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::UpdateGeoParams(payload)] if !payload.params.enabled && payload.params.origin_lat.is_none()),
        "update-geo-params inverts to itself with the base disabled, origin-less facet, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-geo-params/enables-georeferencing-with-an-origin: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: this leaf's FATAL invariant is the strictest in the tree — three
/// strictly-positive finite distances, a non-zero `ortho_max_px`, and (when present) a latitude in
/// [-90, 90] and longitude in [-180, 180]. It is checked BEFORE the no-op guard.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_in_range_origin_invariant() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-geo-params/enables-georeferencing-with-an-origin declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "positive distances, a non-zero ortho resolution and an in-range origin raise no mutation.invariant, got {:?}", produced.messages());
    let params = produced.diff().params.as_ref().expect("update-geo-params writes the params field");
    assert_eq!(params.geo.ortho_max_px, 8192, "the params delta carries the new geo facet");
    assert!(produced.diff().results.is_none(), "update-geo-params writes params alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-geo-params/enables-georeferencing-with-an-origin: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-geo-params/enables-georeferencing-with-an-origin: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-geo-params` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-geo-params/enables-georeferencing-with-an-origin: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let params = committed_diff.params.as_ref().expect("update-geo-params' delta is the whole params block");
    assert_eq!(params.geo.origin_lat, Some(52.375), "the committed delta carries the filled-in survey origin");
    assert_eq!(params.sfm, before().params.sfm, "and repeats all seven sibling facets unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-geo-params/enables-georeferencing-with-an-origin: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-geo-params`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-geo-params/enables-georeferencing-with-an-origin: committed diff did not carry before to after");
}
