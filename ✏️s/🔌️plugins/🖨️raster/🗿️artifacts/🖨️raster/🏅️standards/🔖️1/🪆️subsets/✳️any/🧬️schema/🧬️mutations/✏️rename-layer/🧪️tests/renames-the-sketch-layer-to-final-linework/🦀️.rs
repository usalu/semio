//! 🧪️ `rename-layer` fixture — `renames-the-sketch-layer-to-final-linework`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✏️ `rename-layer` touches the layer's DISPLAY name only — the identity field `id` that every
//! other verb addresses by is deliberately left alone, which is what separates it from a
//! delete/create pair.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, layer_name};
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

/// ▶️ Renaming `sketch` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("rename-layer applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "rename-layer/renames-the-sketch-layer-to-final-linework: applied state differs from committed after-snapshot");
    assert_eq!(layer_name(find_layer(&produced.layers, "sketch").expect("the layer is still addressable by its unchanged id")), "Final Linework", "rename-layer/renames-the-sketch-layer-to-final-linework: the display name must be the payload's");
}

/// ↩️ `rename` is its own inverse partner (`📓️taxonomy.md`): the undo step carries the base's
/// prior `name`, read out of `base`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::RenameLayer(restore)] = inverse.as_slice() else { panic!("rename-layer/renames-the-sketch-layer-to-final-linework: the inverse must be exactly one rename-layer step, got {inverse:?}") };
    assert_eq!(restore.layer_id, "sketch", "rename-layer/renames-the-sketch-layer-to-final-linework: the inverse must re-address the same unchanged id");
    assert_eq!(restore.new_name, "Sketch", "rename-layer/renames-the-sketch-layer-to-final-linework: the inverse must carry the base's own prior name");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-layer/renames-the-sketch-layer-to-final-linework: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-layer/renames-the-sketch-layer-to-final-linework: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-layer/renames-the-sketch-layer-to-final-linework: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: a genuinely new name,
/// so the `mutation.no-op` warning branch is not taken.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-layer/renames-the-sketch-layer-to-final-linework declares an applied outcome");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "rename-layer/renames-the-sketch-layer-to-final-linework: renaming to a genuinely different name raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "rename-layer/renames-the-sketch-layer-to-final-linework: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — `name` is the only
/// patch field a rename may write.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "rename-layer/renames-the-sketch-layer-to-final-linework: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = produced.diff().layers.as_ref().expect("rename-layer writes a layers delta");
    assert_eq!(delta.patched.len(), 1, "rename-layer/renames-the-sketch-layer-to-final-linework: exactly one layer is patched");
    assert_eq!(delta.patched[0].id, "sketch", "rename-layer/renames-the-sketch-layer-to-final-linework: the patch is keyed by the layer's UNCHANGED id");
    assert_eq!(delta.patched[0].patch.name.as_deref(), Some("Final Linework"), "rename-layer/renames-the-sketch-layer-to-final-linework: the patch must carry the new name");
    assert!(delta.added.is_empty() && delta.removed.is_empty(), "rename-layer/renames-the-sketch-layer-to-final-linework: a rename is never a delete/create pair");
    assert!(produced.diff().id.is_none(), "rename-layer/renames-the-sketch-layer-to-final-linework: the DOCUMENT id is not what this verb renames");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-layer/renames-the-sketch-layer-to-final-linework: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-layer/renames-the-sketch-layer-to-final-linework: committed diff did not carry before to after");
}
