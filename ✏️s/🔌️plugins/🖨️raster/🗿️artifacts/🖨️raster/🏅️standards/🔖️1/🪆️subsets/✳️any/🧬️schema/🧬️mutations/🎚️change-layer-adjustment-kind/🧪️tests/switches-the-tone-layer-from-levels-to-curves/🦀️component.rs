//! 🧪️ `change-layer-adjustment-kind` fixture — `switches-the-tone-layer-from-levels-to-curves`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎚️ `adjustment_kind` exists ONLY on `RasterLayerNode::Adjustment`, so this fixture pins the
//! `Adjustment` arm of the diff builder's four-way match; the sibling `Pixel` layer in the same
//! document is the one that would have taken the "is not an adjustment layer" error branch.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterDiff, RasterLayerNode, RasterSnapshot};

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

/// ▶️ Switching `tone` from `levels` to `curves` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("change-layer-adjustment-kind applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: applied state differs from committed after-snapshot");
    let Some(RasterLayerNode::Adjustment { adjustment_kind, params, .. }) = find_layer(&produced.layers, "tone") else { panic!("change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: tone must still be an adjustment layer") };
    assert_eq!(adjustment_kind, "curves", "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: the adjustment kind must be the payload's");
    assert!(params.is_empty(), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: changing the KIND must not fabricate params for it");
}

/// ↩️ `change-layer-adjustment-kind` is its own inverse partner: the undo step carries the base's
/// prior `adjustment_kind`, and only ever exists because `base` really holds an `Adjustment`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::ChangeLayerAdjustmentKind(restore)] = inverse.as_slice() else {
        panic!("change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: the inverse must be exactly one change-layer-adjustment-kind step, got {inverse:?}")
    };
    assert_eq!(restore.layer_id, "tone", "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: the inverse must re-address the same layer");
    assert_eq!(restore.new_adjustment_kind, "levels", "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: the inverse must carry the base's own prior kind");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: an `Adjustment` target
/// whose kind genuinely differs, so no `mutation.no-op` warning and no `mutation.target-missing`.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves declares an applied outcome");
    assert!(matches!(find_layer(&before().layers, "tone"), Some(RasterLayerNode::Adjustment { .. })), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: the target must really be an adjustment layer");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: a genuinely different kind on an adjustment layer raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins that
/// `adjustmentKind` is the ONLY patch field this verb is allowed to write.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let delta = produced.diff().layers.as_ref().expect("change-layer-adjustment-kind writes a layers delta");
    assert_eq!(delta.patched.len(), 1, "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: exactly one layer is patched");
    assert_eq!(delta.patched[0].id, "tone", "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: the patch must address the adjustment layer, not its pixel sibling");
    assert_eq!(delta.patched[0].patch.adjustment_kind.as_deref(), Some("curves"), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: the patch must carry the new kind");
    assert_eq!(
        (delta.patched[0].patch.width, delta.patched[0].patch.height),
        (None, None),
        "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: width/height are rejected outright on an Adjustment — the patch must leave them unset"
    );
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-adjustment-kind/switches-the-tone-layer-from-levels-to-curves: committed diff did not carry before to after");
}
