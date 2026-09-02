//! 🧪️ `resize-layer` fixture — `resizes-the-canvas-layer-to-256-by-128`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 📐 `width`/`height` exist ONLY on `RasterLayerNode::Pixel`, so this fixture pins the `Pixel`
//! arm of the diff builder's five-way match — and a deliberately NON-square target, so a builder
//! that silently swapped the two extents could not pass.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::find_layer;
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

/// ▶️ Resizing `canvas` to 256x128 carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("resize-layer applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "resize-layer/resizes-the-canvas-layer-to-256-by-128: applied state differs from committed after-snapshot");
    let Some(RasterLayerNode::Pixel { width, height, transform, .. }) = find_layer(&produced.layers, "canvas") else { panic!("resize-layer/resizes-the-canvas-layer-to-256-by-128: canvas must still be a pixel layer") };
    assert_eq!((*width, *height), (Some(256), Some(128)), "resize-layer/resizes-the-canvas-layer-to-256-by-128: the extent must be the payload's, width first");
    assert_eq!((transform.scale_x, transform.scale_y), (1.0, 1.0), "resize-layer/resizes-the-canvas-layer-to-256-by-128: resizing the PIXEL EXTENT must not touch the transform's scale");
}

/// ↩️ `resize-layer` is its own inverse partner: the undo step carries the base's prior extent,
/// defaulting to 512 exactly the way `apply_layer_patch` does when the field was unset.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::ResizeLayer(restore)] = inverse.as_slice() else { panic!("resize-layer/resizes-the-canvas-layer-to-256-by-128: the inverse must be exactly one resize-layer step, got {inverse:?}") };
    assert_eq!(restore.layer_id, "canvas", "resize-layer/resizes-the-canvas-layer-to-256-by-128: the inverse must re-address the same layer");
    assert_eq!((restore.new_width, restore.new_height), (512, 512), "resize-layer/resizes-the-canvas-layer-to-256-by-128: the inverse must carry the base's own prior extent");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "resize-layer/resizes-the-canvas-layer-to-256-by-128: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-layer/resizes-the-canvas-layer-to-256-by-128: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "resize-layer/resizes-the-canvas-layer-to-256-by-128: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: both extents are
/// non-zero, so the `mutation.invariant` fatal is not taken, and they genuinely differ, so the
/// `mutation.no-op` warning is not either.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "resize-layer/resizes-the-canvas-layer-to-256-by-128 declares an applied outcome");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "resize-layer/resizes-the-canvas-layer-to-256-by-128: a positive, genuinely-new extent on a pixel layer raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "resize-layer/resizes-the-canvas-layer-to-256-by-128: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — `width` and
/// `height` together, and nothing else.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "resize-layer/resizes-the-canvas-layer-to-256-by-128: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = produced.diff().layers.as_ref().expect("resize-layer writes a layers delta");
    assert_eq!(delta.patched.len(), 1, "resize-layer/resizes-the-canvas-layer-to-256-by-128: exactly one layer is patched");
    assert_eq!((delta.patched[0].patch.width, delta.patched[0].patch.height), (Some(256), Some(128)), "resize-layer/resizes-the-canvas-layer-to-256-by-128: the patch must carry both extents, unswapped");
    assert_eq!(delta.patched[0].patch.adjustment_kind, None, "resize-layer/resizes-the-canvas-layer-to-256-by-128: `adjustmentKind` is rejected outright on a Pixel — the patch must leave it unset");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "resize-layer/resizes-the-canvas-layer-to-256-by-128: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "resize-layer/resizes-the-canvas-layer-to-256-by-128: committed diff did not carry before to after");
}
