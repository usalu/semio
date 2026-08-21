//! 🧪️ `replace-mesh-result` fixture — `swaps-in-an-imported-untextured-mesh`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::remodel::mutations::{apply_remodel_mutation, inverse_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::{RemodelDiff, RemodelSnapshot};
use crate::artifacts::remodel::MeshSource;

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

/// ▶️ `results.mesh` is never `Option` — it is replaced, never cleared. The payload carries an
/// already-minted composed mesh CHILD handle, so this leaf stores it verbatim instead of hashing
/// any geometry itself; the texture reference and watertight report go with it.
#[semio_framework_async_macros::async_test]
async fn stores_the_payload_child_handle_and_clears_the_texture_reference() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("replace-mesh-result applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "replace-mesh-result/swaps-in-an-imported-untextured-mesh: applied state differs from committed after-snapshot");
    assert_eq!(applied.results.mesh.source, MeshSource::Imported, "the provenance moves away from the base reconstructed mesh");
    assert_eq!(applied.results.mesh.mesh.child_id, "remodel-mesh-2f6c81b0d4a37e59", "the payload's composed child handle is stored verbatim, not re-minted");
    assert_eq!(applied.results.mesh.texture_asset_id, None, "the base texture reference is cleared by the wholesale replace");
    assert_eq!(applied.results.mesh.watertight, None, "the base watertight report is cleared by the wholesale replace");
    assert_eq!(applied.assets, before().assets, "the asset the cleared texture reference pointed at is NOT deleted");
}

/// ↩️ The inverse is the same verb carrying the captured base mesh record, boxed.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_reconstructed_textured_mesh() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::ReplaceMeshResult(payload)] if payload.mesh.source == MeshSource::Reconstructed && payload.mesh.texture_asset_id.as_deref() == Some("asset-a")),
        "replace-mesh-result inverts to itself carrying the captured base mesh record, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-mesh-result/swaps-in-an-imported-untextured-mesh: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`: the payload differs from the base mesh record, so the `mutation.no-op`
/// warning — this leaf's only guard — stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_has_only_a_no_op_guard() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "replace-mesh-result/swaps-in-an-imported-untextured-mesh declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "a genuinely different mesh record raises no mutation.no-op, got {:?}", produced.messages());
    let results = produced.diff().results.as_ref().expect("replace-mesh-result writes the results field");
    assert_eq!(results.mesh.source, MeshSource::Imported, "the results delta carries the new mesh record");
    assert!(produced.diff().assets.is_none(), "replace-mesh-result writes results alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-mesh-result/swaps-in-an-imported-untextured-mesh: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-mesh-result/swaps-in-an-imported-untextured-mesh: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `replace-mesh-result` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "replace-mesh-result/swaps-in-an-imported-untextured-mesh: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let results = committed_diff.results.as_ref().expect("replace-mesh-result's delta is the whole results block");
    assert_eq!(results.mesh.source, MeshSource::Imported, "the committed delta carries the imported mesh record");
    assert_eq!(results.qc, before().results.qc, "and repeats every results sibling unchanged");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-mesh-result/swaps-in-an-imported-untextured-mesh: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `replace-mesh-result`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "replace-mesh-result/swaps-in-an-imported-untextured-mesh: committed diff did not carry before to after");
}
