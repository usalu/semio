//! 🧪️ `reorder-layers` fixture — `lifts-the-caption-layer-out-of-the-frame-group`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🔀 `reorder-layers` is a LIST reposition, never a spatial one (the spatial verb is
//! `move-layer`). This case crosses a tree BOUNDARY — out of the `frame` group and to the document
//! root — which is exactly what `RasterLayersDelta.moved`'s `(parentId, index)` address exists for.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, layer_transform, locate_layer};
use crate::artifacts::raster::{RasterDiff, RasterLayerNode, RasterSnapshot};

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

/// ▶️ Lifting `caption` to the document root carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("reorder-layers applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: applied state differs from committed after-snapshot");
    assert_eq!(locate_layer(&produced.layers, "caption"), Some((None, 0)), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the layer must land at the payload's tree address");
    let Some(RasterLayerNode::Group { children, .. }) = find_layer(&produced.layers, "frame") else { panic!("reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the emptied group must survive the move") };
    assert!(children.is_empty(), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the former parent must lose the child, not keep a copy of it");
    let transform = layer_transform(find_layer(&produced.layers, "caption").expect("caption is present"));
    assert_eq!((transform.x, transform.y), (0.0, 0.0), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: a LIST reposition must never touch the layer's spatial transform");
}

/// ↩️ `reorder-layers` is its own inverse partner: the undo step carries the layer's PRE-move tree
/// address, read out of `base` via `locate_layer` — a payload-derived inverse could not know it.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::ReorderLayers(restore)] = inverse.as_slice() else { panic!("reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the inverse must be exactly one reorder-layers step, got {inverse:?}") };
    assert_eq!(restore.layer_id, "caption", "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the inverse must re-address the same layer");
    assert_eq!((restore.parent_id.as_deref(), restore.index), (Some("frame"), 0), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the inverse must carry the base's own pre-move address, back inside the group");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: the target address
/// genuinely differs from the current one, so the `mutation.no-op` warning branch is not taken.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group declares an applied outcome");
    assert_eq!(locate_layer(&before().layers, "caption"), Some((Some("frame".to_string()), 0)), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the before address must differ from the payload's, or the no-op warning would fire");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: a genuine reposition raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — ONE `layers.moved`
/// entry, never a clone-mutate-rediff of the whole snapshot and never an add/remove pair.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(produced.diff().artifact.is_none(), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: a reposition must never fall back to a whole-artifact replacement");
    let delta = produced.diff().layers.as_ref().expect("reorder-layers writes a layers delta");
    assert_eq!(delta.moved.len(), 1, "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: exactly one layer is moved");
    assert_eq!(delta.moved[0].id, "caption", "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: the move must address the layer, not its former parent");
    assert_eq!((delta.moved[0].parent_id.as_deref(), delta.moved[0].index), (None, 0), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: `parentId: null` is the document root");
    assert!(delta.added.is_empty() && delta.removed.is_empty() && delta.patched.is_empty(), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: a reposition is never expressed as an add/remove pair");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-layers/lifts-the-caption-layer-out-of-the-frame-group: committed diff did not carry before to after");
}
