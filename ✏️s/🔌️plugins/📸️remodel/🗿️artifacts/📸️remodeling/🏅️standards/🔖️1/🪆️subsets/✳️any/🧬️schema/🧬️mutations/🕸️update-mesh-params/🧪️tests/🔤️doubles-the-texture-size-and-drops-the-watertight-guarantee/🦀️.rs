//! 🧪️ `update-mesh-params` fixture — `🔤️doubles-the-texture-size-and-drops-the-watertight-guarantee`.
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

/// ▶️ The three watertight-guarantee knobs (`guarantee_watertight`, the hole-fill budget and the
/// self-intersection check) travel in the same facet as the TSDF and texture settings; turning the
/// guarantee off does NOT clear the watertight report already recorded on the mesh result.
#[semio_framework_async_macros::async_test]
async fn drops_the_guarantee_without_clearing_the_recorded_watertight_report() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("update-mesh-params applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee: applied state differs from committed after-snapshot");
    assert_eq!(applied.params.mesh.texture_size, 4096, "the doubled texture size is written");
    assert!(!applied.params.mesh.guarantee_watertight, "the watertight guarantee is switched off");
    assert!(applied.params.mesh.self_intersection_check, "the self-intersection check is switched on");
    assert_eq!(applied.params.mesh.tsdf_voxel_size_mm, 2.5, "the halved TSDF voxel size is written");
    assert!(applied.results.mesh.watertight.is_some(), "the watertight report already on the mesh result survives the params change");
}

/// ↩️ The inverse is the same verb carrying the captured base facet.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_base_two_thousand_forty_eight_texture() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(matches!(inverse.as_slice(), [RemodelingMutation::UpdateMeshParams(payload)] if payload.params.texture_size == 2048 && payload.params.guarantee_watertight), "update-mesh-params inverts to itself with the base facet, got {inverse:?}");
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the no-op guard runs first, then the FATAL invariant demanding a finite
/// TSDF voxel size AND truncation — the only two numbers this leaf validates.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_clears_the_finite_tsdf_invariant() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a finite TSDF voxel size and truncation raise no mutation.invariant, got {:?}", produced.messages());
    let params = produced.diff().params.as_ref().expect("update-mesh-params writes the params field");
    assert_eq!(params.mesh.hole_fill_max_boundary_verts, 1024, "the params delta carries the new mesh facet");
    assert!(produced.diff().results.is_none(), "update-mesh-params writes params alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `update-mesh-params` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let params = committed_diff.params.as_ref().expect("update-mesh-params' delta is the whole params block");
    assert_eq!(params.mesh.texture_size, 4096, "the committed delta carries the doubled texture size");
    assert_eq!(params.motion, before().params.motion, "and repeats all seven sibling facets unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `update-mesh-params`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "update-mesh-params/doubles-the-texture-size-and-drops-the-watertight-guarantee: committed diff did not carry before to after");
}
