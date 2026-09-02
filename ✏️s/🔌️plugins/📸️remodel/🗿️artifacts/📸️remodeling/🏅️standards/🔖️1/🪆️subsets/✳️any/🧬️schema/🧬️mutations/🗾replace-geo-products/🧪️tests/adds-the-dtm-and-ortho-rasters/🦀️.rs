//! 🧪️ `replace-geo-products` fixture — `adds-the-dtm-and-ortho-rasters`.
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

/// ▶️ All three raster references are replaced as one `Option<GeoProducts>` record; the asset
/// keys they name are NOT required to exist in `assets`.
#[semio_framework_async_macros::async_test]
async fn fills_in_the_two_absent_raster_references() {
    let applied = apply_remodeling_mutation(&before(), &mutation()).expect("replace-geo-products applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-geo-products/adds-the-dtm-and-ortho-rasters: applied state differs from committed after-snapshot");
    let geo = applied.results.geo.as_ref().expect("the geo products record is present");
    assert_eq!(geo.dtm_asset_id.as_deref(), Some("asset-dtm"), "the previously null DTM reference is filled in");
    assert_eq!(geo.ortho_asset_id.as_deref(), Some("asset-ortho"), "the previously null ortho reference is filled in");
    assert_eq!(geo.dsm_asset_id.as_deref(), Some("asset-dsm"), "the payload repeats the base DSM reference, so it survives the wholesale replace");
    assert_eq!(applied.assets, before().assets, "the leaf never validates or mints the assets the references name");
    assert_eq!(applied.params.geo, before().params.geo, "writing derived rasters never enables the geo params");
}

/// ↩️ The inverse is the same verb carrying the captured base record.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_dsm_only_base_record() {
    let base = before();
    let inverse = inverse_remodeling_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelingMutation::ReplaceGeoProducts(payload)] if payload.geo.as_ref().is_some_and(|geo| geo.dtm_asset_id.is_none() && geo.ortho_asset_id.is_none())),
        "replace-geo-products inverts to itself carrying the captured DSM-only base record, got {inverse:?}"
    );
    let mut snapshot = apply_remodeling_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodeling_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-geo-products/adds-the-dtm-and-ortho-rasters: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`. Like `replace-trajectory` and `replace-qc`, this leaf rejects a
/// both-null clear with `mutation.target-missing`; a non-null payload never reaches that branch.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_never_reaches_the_nothing_to_clear_rejection() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-geo-products/adds-the-dtm-and-ortho-rasters declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a non-null, genuinely different geo record raises nothing, got {:?}", produced.messages());
    let results = produced.diff().results.as_ref().expect("replace-geo-products writes the results field");
    assert!(results.geo.as_ref().is_some_and(|geo| geo.ortho_asset_id.is_some()), "the results delta carries the new geo record");
    assert!(produced.diff().assets.is_none(), "replace-geo-products writes results alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelingSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-geo-products/adds-the-dtm-and-ortho-rasters: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-geo-products/adds-the-dtm-and-ortho-rasters: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-geo-products` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-geo-products/adds-the-dtm-and-ortho-rasters: produced diff differs from the committed 🔺️diff/🔣️.json");
    let committed_diff: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let results = committed_diff.results.as_ref().expect("replace-geo-products' delta is the whole results block");
    assert!(results.geo.as_ref().is_some_and(|geo| geo.ortho_asset_id.is_some()), "the committed delta carries the completed geo record");
    assert_eq!(results.qc, before().results.qc, "and repeats every results sibling unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelingDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-geo-products/adds-the-dtm-and-ortho-rasters: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-geo-products`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelingDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelingDiff as protocol::MutationDiff<RemodelingSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-geo-products/adds-the-dtm-and-ortho-rasters: committed diff did not carry before to after");
}
