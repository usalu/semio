//! 🖼️ 🖼️ Raster play app commands command — `delete-layer`.

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
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: String,
}

/// 🕹️ No longer prunes selection here (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
/// the `"layers"` domain's selection is framework-owned `InteractionState` now, not this config —
/// `RasterConfigMutation::SetSelection` is deleted. `"layers"` is declared `HierarchyProvider::Flat`,
/// so a deleted-but-still-selected id is a documented, framework-level gap (Flat domains are never
/// auto-pruned by `validate_state`; see the ticket's `w3b-summary.md`), not something this command
/// can restore without re-declaring the domain as `Topology`.
pub fn handle(payload: &DeleteLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    if find_layer(&document.layers, &payload.layer_id).is_none() {
        return Ok(Emit::default());
    }
    Ok(Emit { artifact_mutations: vec![RasterMutation::DeleteLayer(layer_delete::mutation::DeleteLayer { layer_id: payload.layer_id.clone() })], ..Default::default() })
}
