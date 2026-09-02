//! 🧪️ `add-layer-asset` fixture — `declines-to-reattach-an-asset-already-on-the-document`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ⚠️ Why this tree pins the NO-OP APPLIED branch rather than a real attachment: an applied
//! `add-layer-asset` runs the payload's bytes through `crate::artifacts::raster::mint_raster_asset_child`,
//! which mints the composed child handle as `format!("raster-asset-{hash:016x}")` from a
//! `std::collections::hash_map::DefaultHasher` digest of the decoded `SemioImageSnapshot`'s pack
//! bytes. A hand-authored `➡️after` would therefore have to forge a value from `std`'s deliberately
//! unspecified default hasher. The `mutation.no-op` guard below is a REAL branch of this leaf's own
//! diff builder (`base.assets.contains_key`), reached before any minting happens: the diff is the
//! artifact's `Default`, `➡️after` equals `⬅️before`, and no handle is re-minted.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::{RasterDiff, RasterSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> RasterSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> RasterSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> RasterMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A declined re-attach applies cleanly and leaves the document — and its committed child handle
/// — exactly as committed in `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let produced = apply_raster_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(produced, expected_after(), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: applied state differs from committed after-snapshot");
    assert_eq!(produced.assets, base.assets, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: a declined re-attach must not re-mint the committed content-addressed handle");
    assert_eq!(produced.assets.len(), 1, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: the asset map must not grow a second entry for the same key");
}

/// ↩️ This verb's inverse is BASE-derived, and deliberately so: over an already-present key an `add`
/// is an OVERWRITE, so undoing it means re-adding the PRIOR asset — never the destructive
/// `remove-layer-asset` a naive "content missing ⇒ treat as new" read would emit. With the handle
/// present but the in-process working-scene cache cold (the documented staleness gap), it fails
/// soft to no inverse steps at all.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    assert!(inverse.is_empty(), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: a present handle over a cold cache must fail soft to an empty inverse, got {inverse:?}");
    assert!(!inverse.iter().any(|step| matches!(step, RasterMutation::RemoveLayerAsset(_))), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: an unresolvable prior asset must never invert to a destructive removal");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: APPLIED with one
/// `mutation.no-op` WARNING — a no-op is never a rejection, the document stays valid.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document declares an applied outcome");
    assert!(before().assets.contains_key("cover-art"), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: the before-snapshot must already carry the key, or the no-op guard would not fire");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: an already-attached asset is reported as a no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: a no-op is a WARNING — an Error would forbid the empty diff from applying");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries its messages");
    assert_eq!(declared.len(), 1, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: the declared message list must match the emitted one");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: the declared level must be warn");
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: declared applied but the mutation was rejected");
}

/// 🔺️ The delta this mutation produces is exactly the committed diff: the artifact's `Default`
/// — every field `null`, `assets` explicitly untouched, so the minting path is never entered.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(produced.diff(), &RasterDiff::default(), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: a no-op add must produce the artifact's Default diff");
    assert!(produced.diff().assets.is_none(), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: the assets delta is what re-mints a handle — it must stay unwritten");
    assert!(produced.diff().layers.is_none(), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: attaching an asset never edits the layer tree, not even to fix up `imageKey`");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: committed diff did not carry before to after");
    assert_eq!(produced.assets, before().assets, "add-layer-asset/declines-to-reattach-an-asset-already-on-the-document: applying the committed diff must not mint a handle either");
}
