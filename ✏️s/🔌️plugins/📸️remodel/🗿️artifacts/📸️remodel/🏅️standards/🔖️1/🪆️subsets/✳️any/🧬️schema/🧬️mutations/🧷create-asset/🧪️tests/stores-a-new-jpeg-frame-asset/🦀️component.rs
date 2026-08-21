//! 🧪️ `create-asset` fixture — `stores-a-new-jpeg-frame-asset`.
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

/// ▶️ The raw `ImageAsset` payload never lands in the document: the diff mints a composed
/// `s.stdio.semio/v1/image` CHILD handle and stores that instead, keyed by the payload key.
/// `image/jpeg` does not decode through the semio-image bridge, so the handle is the RAW-bytes
/// content hash of `(mime, data)`.
#[semio_framework_async_macros::async_test]
async fn stores_a_minted_child_handle_rather_than_the_raw_bytes() {
    let applied = apply_remodel_mutation(&before(), &mutation()).expect("create-asset applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "create-asset/stores-a-new-jpeg-frame-asset: applied state differs from committed after-snapshot");
    let minted = applied.assets.get("asset-b").expect("the new key is present in the assets map");
    assert_eq!(minted.child_id, "remodel-asset-75b20f8d69a86e9a", "the child id is the deterministic content hash of the raw (mime, data) pair");
    assert_eq!(minted.target.artifact_id, "asset-b-image", "the child target is derived from the asset key");
    assert_eq!(minted.target.dialect.subset, "image", "assets compose the semio image subset");
    assert_eq!(applied.assets.len(), 2, "the pre-existing asset-a handle is preserved beside the new key");
    assert_eq!(applied.streams, before().streams, "create-asset never wires the new asset into any stream frame");
}

/// ↩️ For a key that is absent from `base`, the inverse is exactly one `delete-asset`.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_single_delete_of_asset_b() {
    let base = before();
    let inverse = inverse_remodel_mutation(&base, &mutation());
    assert!(
        matches!(inverse.as_slice(), [RemodelMutation::DeleteAsset(payload)] if payload.key == "asset-b"),
        "create-asset's inverse for a fresh key is one delete-asset for that key, got {inverse:?}"
    );
    let mut snapshot = apply_remodel_mutation(&base, &mutation()).expect("forward applies");
    for step in &inverse {
        snapshot = apply_remodel_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-asset/stores-a-new-jpeg-frame-asset: inverse did not restore the before-snapshot");
}

/// 🎯️ Declared `applied`. This leaf is deliberately guard-free: it upserts, so an existing key is
/// an overwrite rather than a `mutation.duplicate-id` rejection, and it has no no-op warning.
#[semio_framework_async_macros::async_test]
async fn declared_applied_outcome_is_an_unguarded_upsert() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared["status"], "applied", "create-asset/stores-a-new-jpeg-frame-asset declares an applied outcome");
    let produced = produced();
    assert!(produced.messages().is_empty(), "create-asset raises no diagnostics at all, got {:?}", produced.messages());
    let assets = produced.diff().assets.as_ref().expect("create-asset writes the assets field");
    assert_eq!(assets.len(), 2, "the assets delta REPLACES the whole map, so it carries the pre-existing key too");
    assert!(produced.diff().results.is_none() && produced.diff().streams.is_none(), "create-asset writes assets alone");
}

/// 🔣️ The committed snapshots and the committed mutation are already canonical: decode→encode is a
/// fixed point, so `fixtures generate` derives the other encodings from stable bytes.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RemodelSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-asset/stores-a-new-jpeg-frame-asset: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-asset/stores-a-new-jpeg-frame-asset: committed mutation JSON is not canonical");
}

/// 🔺️ The sparse delta `create-asset` produces is EXACTLY the committed diff — the
/// load-bearing assertion of the whole fixture, because it pins which fields this leaf is allowed to
/// touch rather than merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    let encoded = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "create-asset/stores-a-new-jpeg-frame-asset: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let committed_diff: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let assets = committed_diff.assets.as_ref().expect("create-asset's delta is the whole assets map");
    assert_eq!(assets.len(), 2, "the committed delta is the whole post-insert map — a replace, not a single-entry insert");
    assert_eq!(assets.get("asset-b").expect("the new key is in the delta").child_id, "remodel-asset-75b20f8d69a86e9a", "pinning the minted content-addressed child id");
}

/// 🔣️ The committed diff is itself canonical and decodes back into `RemodelDiff`, whose seventeen
/// `Option` fields carry no `skip_serializing_if` — every untouched field must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-asset/stores-a-new-jpeg-frame-asset: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields `after` — the delta is a complete
/// description of `create-asset`'s change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RemodelDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let applied = <RemodelDiff as protocol::MutationDiff<RemodelSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(applied, expected_after(), "create-asset/stores-a-new-jpeg-frame-asset: committed diff did not carry before to after");
}
