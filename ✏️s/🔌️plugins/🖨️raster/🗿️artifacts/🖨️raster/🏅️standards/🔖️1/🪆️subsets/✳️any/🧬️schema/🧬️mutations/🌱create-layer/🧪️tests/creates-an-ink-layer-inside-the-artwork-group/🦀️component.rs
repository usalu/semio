//! 🧪️ `create-layer` fixture — `creates-an-ink-layer-inside-the-artwork-group`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🌱 The insertion is TREE-ADDRESSED: `parentId = "artwork"` puts the new node inside a nested
//! `Group`, which is exactly what `RasterLayersDelta.added` exists to express sparsely (the root
//! layer list is left byte-identical).

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, locate_layer};
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

/// ▶️ Creating `ink` inside `artwork` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("create-layer applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: applied state differs from committed after-snapshot");
    assert_eq!(produced.layers.len(), 1, "create-layer/creates-an-ink-layer-inside-the-artwork-group: a nested insertion must not grow the ROOT layer list");
    assert_eq!(locate_layer(&produced.layers, "ink"), Some((Some("artwork".to_string()), 1)), "create-layer/creates-an-ink-layer-inside-the-artwork-group: the new layer must land at the payload's tree address");
    assert!(find_layer(&produced.layers, "sketch").is_some(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: the pre-existing sibling must survive");
}

/// ↩️ `create`'s inverse partner is `delete`, addressed by the created layer's own id — derived
/// from the PAYLOAD, never from `base` (the id is not in `base` yet).
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::DeleteLayer(restore)] = inverse.as_slice() else { panic!("create-layer/creates-an-ink-layer-inside-the-artwork-group: the inverse must be exactly one delete-layer step, got {inverse:?}") };
    assert_eq!(restore.layer_id, "ink", "create-layer/creates-an-ink-layer-inside-the-artwork-group: the inverse must delete the id the payload introduced");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-layer/creates-an-ink-layer-inside-the-artwork-group: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-layer/creates-an-ink-layer-inside-the-artwork-group: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-layer/creates-an-ink-layer-inside-the-artwork-group: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: a fresh id under a real
/// `Group` parent, so neither the `mutation.duplicate-id` nor the `mutation.invariant` fatal fires.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-layer/creates-an-ink-layer-inside-the-artwork-group declares an applied outcome");
    assert!(find_layer(&before().layers, "ink").is_none(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: the before-snapshot must not already carry the new id, or the duplicate-id fatal would fire");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: a fresh id under a real group raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — one
/// `layers.added` insertion carrying its own tree address, never a whole-snapshot capture.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "create-layer/creates-an-ink-layer-inside-the-artwork-group: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(produced.diff().artifact.is_none(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: a creation must never fall back to a whole-artifact replacement");
    let delta = produced.diff().layers.as_ref().expect("create-layer writes a layers delta");
    assert_eq!(delta.added.len(), 1, "create-layer/creates-an-ink-layer-inside-the-artwork-group: exactly one layer is added");
    assert_eq!(delta.added[0].parent_id.as_deref(), Some("artwork"), "create-layer/creates-an-ink-layer-inside-the-artwork-group: the insertion must carry the nested parent id");
    assert_eq!(delta.added[0].index, 1, "create-layer/creates-an-ink-layer-inside-the-artwork-group: the insertion must carry the payload's index");
    assert!(delta.removed.is_empty() && delta.patched.is_empty() && delta.moved.is_empty(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: creating a layer must not remove, patch or move anything");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-layer/creates-an-ink-layer-inside-the-artwork-group: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-layer/creates-an-ink-layer-inside-the-artwork-group: committed diff did not carry before to after");
}
