//! 🧪️ `delete-layer` fixture — `deletes-the-frame-group-and-its-nested-children`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🗑️ Deleting a `Group` CASCADES: `remove_layer_from_tree` lifts out the matched node with its
//! whole subtree, so one `layers.removed` entry silently takes `caption` and `border` with it. That
//! is exactly what makes the committed diff load-bearing here — the end state alone could not tell
//! a cascading removal apart from three separate ones.

use crate::artifacts::raster::mutations::{apply_raster_mutation, inverse_raster_mutation, RasterMutation};
use crate::artifacts::raster::schema::{find_layer, flatten_raster_layers, layer_node_id, locate_layer};
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

/// ▶️ Deleting `frame` carries `before` to exactly the committed `after`, taking its whole subtree.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let produced = apply_raster_mutation(&before(), &mutation()).expect("delete-layer applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-layer/deletes-the-frame-group-and-its-nested-children: applied state differs from committed after-snapshot");
    for cascaded in ["frame", "caption", "border"] {
        assert!(find_layer(&produced.layers, cascaded).is_none(), "delete-layer/deletes-the-frame-group-and-its-nested-children: {cascaded} must be gone — deleting a group cascades into its subtree");
    }
    let surviving: Vec<&str> = flatten_raster_layers(&produced.layers).into_iter().map(layer_node_id).collect();
    assert_eq!(surviving, vec!["backdrop"], "delete-layer/deletes-the-frame-group-and-its-nested-children: only the untargeted sibling may survive");
}

/// ↩️ `delete`'s inverse partner is `create`, and it must capture the FULL removed subtree plus its
/// tree address out of `base` — a payload-derived inverse would only know the id.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = inverse_raster_mutation(&base, &forward);
    let [RasterMutation::CreateLayer(restore)] = inverse.as_slice() else { panic!("delete-layer/deletes-the-frame-group-and-its-nested-children: the inverse must be exactly one create-layer step, got {inverse:?}") };
    assert_eq!((restore.parent_id.as_deref(), restore.index), (None, 1), "delete-layer/deletes-the-frame-group-and-its-nested-children: the inverse must carry the group's own pre-delete address");
    let RasterLayerNode::Group { id, children, .. } = &*restore.layer else { panic!("delete-layer/deletes-the-frame-group-and-its-nested-children: the inverse must carry the removed GROUP node, not a bare placeholder") };
    assert_eq!(id, "frame", "delete-layer/deletes-the-frame-group-and-its-nested-children: the inverse must re-create the deleted id");
    assert_eq!(children.len(), 2, "delete-layer/deletes-the-frame-group-and-its-nested-children: the inverse must carry the whole cascaded subtree, not just the group shell");
    let mut snapshot = apply_raster_mutation(&base, &forward).expect("forward applies");
    for step in &inverse {
        snapshot = apply_raster_mutation(&snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-layer/deletes-the-frame-group-and-its-nested-children: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and so
/// is the committed mutation payload.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: RasterSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-layer/deletes-the-frame-group-and-its-nested-children: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-layer/deletes-the-frame-group-and-its-nested-children: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces: the target really is in
/// the tree, so the `mutation.target-missing` error branch is not taken.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-layer/deletes-the-frame-group-and-its-nested-children declares an applied outcome");
    assert_eq!(locate_layer(&before().layers, "frame"), Some((None, 1)), "delete-layer/deletes-the-frame-group-and-its-nested-children: the target must really be in the before-snapshot");
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-layer/deletes-the-frame-group-and-its-nested-children: deleting a present layer raises no diagnostic, got {:?}", produced.messages());
    assert!(apply_raster_mutation(&before(), &mutation()).is_ok(), "delete-layer/deletes-the-frame-group-and-its-nested-children: declared applied but the mutation was rejected");
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — ONE
/// `layers.removed` id, with the cascade left to apply-time recursion rather than spelled out as
/// three separate removals.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <RasterMutation as protocol::Mutation<RasterSnapshot>>::diff(&mutation(), &before());
    let encoded = serde_json::to_value(produced.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(encoded, committed, "delete-layer/deletes-the-frame-group-and-its-nested-children: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = produced.diff().layers.as_ref().expect("delete-layer writes a layers delta");
    assert_eq!(delta.removed, vec!["frame".to_string()], "delete-layer/deletes-the-frame-group-and-its-nested-children: the cascade must NOT be spelled out — only the addressed group id is removed");
    assert!(delta.added.is_empty() && delta.patched.is_empty() && delta.moved.is_empty(), "delete-layer/deletes-the-frame-group-and-its-nested-children: a deletion must not add, patch or move anything");
    assert!(produced.diff().assets.is_none(), "delete-layer/deletes-the-frame-group-and-its-nested-children: deleting a layer never garbage-collects the asset map");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-layer/deletes-the-frame-group-and-its-nested-children: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: RasterDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <RasterDiff as protocol::MutationDiff<RasterSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-layer/deletes-the-frame-group-and-its-nested-children: committed diff did not carry before to after");
}
