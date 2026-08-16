//! 🖼️ 🖼️ Raster play app commands command — `move-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::editor::raster::layer_id_from_tree_row_id;
use crate::artifacts::raster::schema::{clone_layer, create_layer_of_kind, find_layer, layer_node_id, layer_opacity, layer_transform, layer_visible};
use crate::artifacts::raster::mutations::{change_layer_adjustment_kind, change_layer_blend_mode, change_layer_opacity, change_layer_visible, create_layer, reorder_layers, rename_layer, resize_layer};
use crate::artifacts::raster::mutations::delete_layer as layer_delete;
use crate::artifacts::raster::mutations::move_layer as spatial_move_layer;
use crate::artifacts::raster::op::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "move-layer")]
pub struct MoveLayer {
    pub layer_id: String,
    pub target_row_id: String,
    pub drop_position: String,
}

/// 🌳️ This is the layer-tree DRAG gesture (drop onto/into another row) — a list REPOSITION, so it
/// now emits `reorder-layers`, never the spatial `move-layer` (which is `transform.x`/`.y`).
pub fn handle(payload: &MoveLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    if find_layer(&document.layers, &payload.layer_id).is_none() {
        return Ok(Emit::default());
    }
    let parent_id = layer_id_from_tree_row_id(&payload.target_row_id).and_then(|id| find_layer(&document.layers, &id).and_then(|entry| matches!(entry, RasterLayerNode::Group { .. }).then_some(id)));
    let index = if payload.drop_position == "before" {
        0
    } else if let Some(parent) = &parent_id {
        match find_layer(&document.layers, parent) {
            Some(RasterLayerNode::Group { children, .. }) => children.len(),
            _ => 0,
        }
    } else {
        document.layers.len()
    };
    Ok(Emit::mutations(vec![RasterMutation::ReorderLayers(reorder_layers::mutation::ReorderLayers { layer_id: payload.layer_id.clone(), parent_id, index })]))
}
