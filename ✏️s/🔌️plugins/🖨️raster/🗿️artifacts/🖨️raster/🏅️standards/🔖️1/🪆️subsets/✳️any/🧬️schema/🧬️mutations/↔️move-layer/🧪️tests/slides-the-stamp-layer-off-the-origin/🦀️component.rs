//! 🧪️ `move-layer` fixture — `slides-the-stamp-layer-off-the-origin`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ↔️ `move-layer` is SPATIAL: it writes `transform.x`/`.y` and nothing else. The list-position
//! verb is `reorder-layers`, which this fixture deliberately never reaches — `stamp` keeps index 0
//! throughout.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, layer_transform, locate_layer};
use crate::artifacts::raster::{RasterDiff, RasterSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
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

/// ▶️ Sliding `stamp` to (16, -8) carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("move-layer applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "move-layer/slides-the-stamp-layer-off-the-origin: applied state differs from committed after-snapshot");
    let transform = layer_transform(find_layer(&produced.layers, "stamp").expect("stamp is present"));
    assert_eq!((transform.x, transform.y), (16.0, -8.0), "move-layer/slides-the-stamp-layer-off-the-origin: the layer must sit at the payload's absolute position");
    assert_eq!((transform.scale_x, transform.scale_y, transform.rotation), (1.0, 1.0, 0.0), "move-layer/slides-the-stamp-layer-off-the-origin: a move must not disturb scale or rotation");
    assert_eq!(locate_layer(&produced.layers, "stamp"), Some((None, 0)), "move-layer/slides-the-stamp-layer-off-the-origin: a SPATIAL move must leave the layer's LIST position alone");
}

/// ↩️ `move-layer` is its own inverse partner: the undo step carries the base's prior
/// `transform.x`/`.y`, read out of `base`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::MoveLayer(restore)] = inverse.as_slice() else {
        panic!("move-layer/slides-the-stamp-layer-off-the-origin: the inverse must be exactly one move-layer step, got {inverse:?}")
    };
    assert_eq!(restore.layer_id, "stamp", "move-layer/slides-the-stamp-layer-off-the-origin: the inverse must re-address the same layer");
    assert_eq!((restore.new_x, restore.new_y), (0.0, 0.0), "move-layer/slides-the-stamp-layer-off-the-origin: the inverse must carry the base's own origin position");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "move-layer/slides-the-stamp-layer-off-the-origin: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-layer/slides-the-stamp-layer-off-the-origin: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-layer/slides-the-stamp-layer-off-the-origin: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: finite coordinates that
/// genuinely differ from the base's, so neither the `mutation.invariant` fatal nor the
/// `mutation.no-op` warning fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-layer/slides-the-stamp-layer-off-the-origin declares an applied outcome");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "move-layer/slides-the-stamp-layer-off-the-origin: a finite, genuinely-new position raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "move-layer/slides-the-stamp-layer-off-the-origin: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins that a
/// move writes `transformX`/`transformY` and no other patch field.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "move-layer/slides-the-stamp-layer-off-the-origin: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let delta = produced.diff().layers.as_ref().expect("move-layer writes a layers delta");
    assert!(delta.moved.is_empty(), "move-layer/slides-the-stamp-layer-off-the-origin: `layers.moved` belongs to reorder-layers — a spatial move must never write it");
    assert_eq!(delta.patched.len(), 1, "move-layer/slides-the-stamp-layer-off-the-origin: exactly one layer is patched");
    assert_eq!((delta.patched[0].patch.transform_x, delta.patched[0].patch.transform_y), (Some(16.0), Some(-8.0)), "move-layer/slides-the-stamp-layer-off-the-origin: the patch must carry both new coordinates");
    assert_eq!(delta.patched[0].patch.name, None, "move-layer/slides-the-stamp-layer-off-the-origin: a move must not rename the layer");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-layer/slides-the-stamp-layer-off-the-origin: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-layer/slides-the-stamp-layer-off-the-origin: committed diff did not carry before to after");
}
