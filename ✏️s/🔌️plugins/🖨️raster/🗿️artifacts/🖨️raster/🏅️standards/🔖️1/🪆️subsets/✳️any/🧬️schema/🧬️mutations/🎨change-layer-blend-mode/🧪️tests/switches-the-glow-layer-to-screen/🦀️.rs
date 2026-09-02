//! 🧪️ `change-layer-blend-mode` fixture — `switches-the-glow-layer-to-screen`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎨 `blend_mode` is a free-form `String` shared by all three layer variants, so the diff
//! builder reaches it through a three-arm or-pattern rather than a variant-specific match; this
//! fixture pins that a plain `Pixel` layer really does take that shared path.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, layer_blend_mode, layer_opacity};
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

/// ▶️ Switching `glow` to `screen` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("change-layer-blend-mode applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-blend-mode/switches-the-glow-layer-to-screen: applied state differs from committed after-snapshot");
    let layer = find_layer(&produced.layers, "glow").expect("glow is present");
    assert_eq!(layer_blend_mode(layer), "screen", "change-layer-blend-mode/switches-the-glow-layer-to-screen: the blend mode must be the payload's");
    assert_eq!(layer_opacity(layer), 1.0, "change-layer-blend-mode/switches-the-glow-layer-to-screen: compositing MODE and compositing STRENGTH are separate verbs — opacity must be untouched");
}

/// ↩️ `change-layer-blend-mode` is its own inverse partner: the undo step carries the base's prior
/// `blend_mode` string, read out of `base`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::ChangeLayerBlendMode(restore)] = inverse.as_slice() else { panic!("change-layer-blend-mode/switches-the-glow-layer-to-screen: the inverse must be exactly one change-layer-blend-mode step, got {inverse:?}") };
    assert_eq!(restore.layer_id, "glow", "change-layer-blend-mode/switches-the-glow-layer-to-screen: the inverse must re-address the same layer");
    assert_eq!(restore.new_blend_mode, "normal", "change-layer-blend-mode/switches-the-glow-layer-to-screen: the inverse must carry the base's own prior blend mode");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-layer-blend-mode/switches-the-glow-layer-to-screen: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-layer-blend-mode/switches-the-glow-layer-to-screen: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-layer-blend-mode/switches-the-glow-layer-to-screen: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: a genuinely different
/// blend mode, so the `mutation.no-op` warning branch is not taken.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-layer-blend-mode/switches-the-glow-layer-to-screen declares an applied outcome");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-layer-blend-mode/switches-the-glow-layer-to-screen: a genuinely different blend mode raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "change-layer-blend-mode/switches-the-glow-layer-to-screen: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — `blendMode` is the
/// only patch field this verb may write.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "change-layer-blend-mode/switches-the-glow-layer-to-screen: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = produced.diff().layers.as_ref().expect("change-layer-blend-mode writes a layers delta");
    assert_eq!(delta.patched.len(), 1, "change-layer-blend-mode/switches-the-glow-layer-to-screen: exactly one layer is patched");
    assert_eq!(delta.patched[0].patch.blend_mode.as_deref(), Some("screen"), "change-layer-blend-mode/switches-the-glow-layer-to-screen: the patch must carry the new blend mode");
    assert_eq!(delta.patched[0].patch.opacity, None, "change-layer-blend-mode/switches-the-glow-layer-to-screen: the patch must leave `opacity` to change-layer-opacity");
    assert!(produced.diff().locale.is_none(), "change-layer-blend-mode/switches-the-glow-layer-to-screen: a blend-mode change never writes a config-class field");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-layer-blend-mode/switches-the-glow-layer-to-screen: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-blend-mode/switches-the-glow-layer-to-screen: committed diff did not carry before to after");
}
