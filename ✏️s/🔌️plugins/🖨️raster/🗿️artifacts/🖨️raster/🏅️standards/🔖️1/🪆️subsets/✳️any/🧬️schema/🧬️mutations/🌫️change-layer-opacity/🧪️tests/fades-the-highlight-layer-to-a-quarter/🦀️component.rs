//! 🧪️ `change-layer-opacity` fixture — `fades-the-highlight-layer-to-a-quarter`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🌫️ `opacity` is the vocabulary's only `f32` scalar; `0.25` is dyadic, so the committed JSON
//! round-trips through `f32` exactly and the canonical-JSON assertions below are not quietly
//! testing a float-formatting accident.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, layer_opacity, layer_visible};
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

/// ▶️ Fading `highlight` to a quarter carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("change-layer-opacity applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: applied state differs from committed after-snapshot");
    let layer = find_layer(&produced.layers, "highlight").expect("highlight is present");
    assert_eq!(layer_opacity(layer), 0.25, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: the opacity must be the payload's");
    assert!(layer_visible(layer), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: fading is not hiding — `visible` must stay true");
}

/// ↩️ `change-layer-opacity` is its own inverse partner: the undo step carries the base's prior
/// `opacity`, read out of `base`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::ChangeLayerOpacity(restore)] = inverse.as_slice() else { panic!("change-layer-opacity/fades-the-highlight-layer-to-a-quarter: the inverse must be exactly one change-layer-opacity step, got {inverse:?}") };
    assert_eq!(restore.layer_id, "highlight", "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: the inverse must re-address the same layer");
    assert_eq!(restore.new_opacity, 1.0, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: the inverse must carry the base's own prior opacity");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: a finite opacity that
/// genuinely differs, so neither the `mutation.invariant` fatal nor the `mutation.no-op` warning
/// fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter declares an applied outcome");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: a finite, genuinely-new opacity raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — `opacity` is the
/// only patch field this verb may write.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let delta = produced.diff().layers.as_ref().expect("change-layer-opacity writes a layers delta");
    assert_eq!(delta.patched.len(), 1, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: exactly one layer is patched");
    assert_eq!(delta.patched[0].patch.opacity, Some(0.25), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: the patch must carry the new opacity");
    assert_eq!(delta.patched[0].patch.visible, None, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: the patch must leave `visible` to change-layer-visible");
    assert!(produced.diff().brush_opacity.is_none(), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: LAYER opacity and the config-class BRUSH opacity are different fields entirely");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-opacity/fades-the-highlight-layer-to-a-quarter: committed diff did not carry before to after");
}
