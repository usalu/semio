//! 🎬️ 🖼️ Raster play app commands command — `set-active-example`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::{add_layer_asset, create_layer, delete_layer, remove_layer_asset};
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::{layer_node_id, raster_example_document};
use crate::{raster_asset, RasterLayerNode, RasterOwnedMap, RasterSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🔁️ The ordered batch of semantic mutations carrying `current` to `next`. Raster deliberately has
/// no whole-document replace verb (`🎮️commands/📃️document/🦀️.rs` records why `setSnapshot` was
/// deleted), so loading an example is spelled as real, undoable operations: every root layer is
/// deleted first (deleting a `Group` cascades its whole subtree), then the asset pool is re-pointed,
/// then the example's own forest is planted at the root in declaration order — `create-layer` carries
/// a whole `RasterLayerNode` including its `children`, so one operation per root layer is one whole
/// subtree. An asset whose composed child has not been materialized locally is skipped rather than
/// fabricated; its `image_key` then dangles exactly as it does for any wire-decoded document.
///
/// 🚚️ `next` is consumed, never cloned: `RasterLayerNode::clone` (and the derived `RasterSnapshot`
/// drop) trap on a populated `RasterOwnedMap` — the demo's brighten adjustment carries `brightness`/
/// `contrast` params — so every layer MOVES into its `create-layer` operation and the asset map is
/// drained page by page, leaving an empty shell that drops cleanly (react boot of ticket
/// 26/09/05/RASTER-PLUGIN-END-TO-END, 2026-09-16).
fn replace_document_operations(current: &RasterSnapshot, mut next: RasterSnapshot) -> Vec<RasterMutation> {
    let mut operations = Vec::new();
    for layer in &current.layers {
        operations.push(RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: layer_node_id(layer).to_string() }));
    }
    for asset_id in current.assets.keys() {
        operations.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: asset_id.clone() }));
    }
    for (asset_id, _) in next.assets.iter() {
        if let Some(asset) = raster_asset(&next.assets, asset_id) {
            operations.push(RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: asset_id.to_owned(), asset }));
        }
    }
    drain_owned_map(&mut next.assets);
    for (index, layer) in std::mem::take(&mut next.layers).into_iter().enumerate() {
        operations.push(RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index, layer: Box::new(layer) }));
    }
    operations
}

/// 🧹️ Empties one owned map entry by entry and releases every page backing, so the map reaches
/// `Drop` in the only state its guard admits. `ArtifactChild`/`DslValue` entries carry no drop guard
/// of their own.
fn drain_owned_map<V>(map: &mut RasterOwnedMap<V>) {
    while map.take_last_entry().is_some() {}
    while let Some(page) = map.take_empty_page_backing() {
        page.release();
    }
}

/// 🧹️ Dismantles a layer forest that is NOT being planted (the no-op path below): an adjustment's
/// `params` map must be drained before the node drops; groups recurse.
fn release_layer_forest(layers: Vec<RasterLayerNode>) {
    for layer in layers {
        match layer {
            RasterLayerNode::Adjustment { mut params, .. } => drain_owned_map(&mut params),
            RasterLayerNode::Group { children, .. } => release_layer_forest(children),
            RasterLayerNode::Pixel { .. } => {}
        }
    }
}

/// 🎬️ Loads one registered example over the open document. An id this subset does not register is a
/// no-op rather than a fault — the navbar switcher is free-text on the wire, and an unknown id must
/// not destroy the open document.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let Some(mut example) = raster_example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };
    // 🟰 The shell replays `setActiveExample` on every boot; over a document that already carries
    // the example's forest, re-planting would delete and recreate every layer for no change. Only
    // the layer forest is compared: the store keeps its own `id`/`title`, and the example's asset
    // handles are planted only when materialized locally (never, today). The unused example is
    // dismantled, never dropped populated (see `release_layer_forest`).
    if doc.snapshot.layers == example.layers {
        drain_owned_map(&mut example.assets);
        release_layer_forest(std::mem::take(&mut example.layers));
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(replace_document_operations(doc.snapshot, example)))
}
