//! 🧪️ `change-layer-visible` fixture — `🟠️hides-the-overlay-layer`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 👁️ This verb writes exactly ONE boolean through `diff_patch_layer`: `overlay.visible`
//! flips to `false` while `backdrop` — and every other field of `overlay` reachable through the
//! same `RasterLayerPatch` — stays untouched.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, layer_visible};
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

/// ▶️ Hiding `overlay` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("change-layer-visible applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-visible/hides-the-overlay-layer: applied state differs from committed after-snapshot");
    assert!(!layer_visible(find_layer(&produced.layers, "overlay").expect("overlay is still present — hiding is not deleting")), "change-layer-visible/hides-the-overlay-layer: overlay must end up hidden");
    assert!(layer_visible(find_layer(&produced.layers, "backdrop").expect("backdrop is present")), "change-layer-visible/hides-the-overlay-layer: the sibling backdrop must keep its own visibility");
}

/// ↩️ `change-layer-visible` is its own inverse partner: the undo step is the SAME verb carrying
/// the base's prior `visible`, read out of `base` (never out of the payload).
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::ChangeLayerVisible(restore)] = inverse.as_slice() else { panic!("change-layer-visible/hides-the-overlay-layer: the inverse must be exactly one change-layer-visible step, got {inverse:?}") };
    assert_eq!(restore.layer_id, "overlay", "change-layer-visible/hides-the-overlay-layer: the inverse must re-address the same layer");
    assert!(restore.new_visible, "change-layer-visible/hides-the-overlay-layer: the inverse must carry the base's own `visible = true`");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-layer-visible/hides-the-overlay-layer: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-layer-visible/hides-the-overlay-layer: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-layer-visible/hides-the-overlay-layer: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: a clean apply with no
/// diagnostic, because `overlay` was genuinely visible before.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-layer-visible/hides-the-overlay-layer declares an applied outcome");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-layer-visible/hides-the-overlay-layer: flipping a genuinely different `visible` must not raise the mutation.no-op warning, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "change-layer-visible/hides-the-overlay-layer: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields the mutation is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "change-layer-visible/hides-the-overlay-layer: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = produced.diff().layers.as_ref().expect("change-layer-visible writes a layers delta");
    assert!(delta.added.is_empty() && delta.removed.is_empty() && delta.moved.is_empty(), "change-layer-visible/hides-the-overlay-layer: this verb patches in place — it never adds, removes or moves a layer");
    assert_eq!(delta.patched.len(), 1, "change-layer-visible/hides-the-overlay-layer: exactly one layer is patched");
    assert_eq!(delta.patched[0].patch.visible, Some(false), "change-layer-visible/hides-the-overlay-layer: the patch must carry the new `visible`");
    assert_eq!(delta.patched[0].patch.opacity, None, "change-layer-visible/hides-the-overlay-layer: visibility and opacity are separate verbs — the patch must leave `opacity` unset");
    assert!(produced.diff().assets.is_none(), "change-layer-visible/hides-the-overlay-layer: a visibility change never touches the asset map");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-layer-visible/hides-the-overlay-layer: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-visible/hides-the-overlay-layer: committed diff did not carry before to after");
}
