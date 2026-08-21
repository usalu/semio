//! 🧪️ `remove-layer-asset` fixture — `rejects-removing-an-asset-the-document-never-attached`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this tree pins a REJECTION branch rather than a real removal: `RasterSnapshot.assets`
//! values are composed `s.stdio.semio.image` CHILD handles whose `child_id` is minted as
//! `format!("raster-asset-{hash:016x}")` from a `std::collections::hash_map::DefaultHasher` digest
//! (`crate::artifacts::raster::mint_and_stash_asset`). An APPLIED removal's inverse is
//! `add-layer-asset`, whose diff-apply RE-MINTS that handle — so a hand-authored `➡️after` that
//! round-tripped through the inverse would require forging a value from `std`'s deliberately
//! unspecified default hasher. The `mutation.target-missing` branch below is reached without ever
//! touching the minting path, and is the same branch this verb really emits in production.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::{RasterDiff, RasterSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> RasterSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RasterSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RasterMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `remove-layer-asset` leaves the document at the committed `after` — in particular
/// the surviving `photo` handle is not re-minted, dropped or reordered.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let produced = apply_raster_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(produced, expected_after(), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: applied state differs from committed after-snapshot");
    assert_eq!(produced.assets, base.assets, "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: a rejected removal must leave every committed child handle byte-identical");
}

/// 🗂️ `remove-layer-asset` is one of only two verbs addressed by ASSET id: it searches the
/// `assets` map, never the layer tree, and reports the missing asset id verbatim.
#[semio_framework_async_macros::async_test]
async fn a_missing_asset_is_reported_by_its_asset_id() {
    let base = before();
    assert!(base.assets.contains_key("photo"), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: the before-snapshot must carry a DIFFERENT asset, so the rejection is about the key and not about an empty map");
    assert!(!base.assets.contains_key("logo"), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: the targeted asset must genuinely be absent");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &RasterDiff::default(), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: a rejecting removal must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: an unattached asset is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: this verb has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["logo".to_string()], "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: the diagnostic names the ASSET id, not the layer that would reference it");
    let semantics = <RasterMutation as protocol::SemanticMutation<RasterSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("remove", "asset", "remove-layer-asset", "RemovedLayerAsset"), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: the fixture must be bound to remove-layer-asset's own descriptor");
}

/// ↩️ This verb's inverse is BASE-derived: it recovers the removed bytes through the working-scene
/// cache accessor. With no such asset attached at all, there is nothing to reattach and the inverse
/// is empty — never a destructive `add-layer-asset` guess.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_asset_to_reattach() {
    let inverse = inverse_raster_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: a rejected removal must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached declares a rejected outcome");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome
        .get("path")
        .and_then(serde_json::Value::as_array)
        .expect("a rejected outcome declares a path")
        .iter()
        .map(|entry| entry.as_str().expect("path segments are strings").to_string())
        .collect();
    assert_eq!(declared_path, message.target, "remove-layer-asset/rejects-removing-an-asset-the-document-never-attached: the declared path must match the emitted target");
}
