//! 🖼️ 🖼️ Raster play app commands command — `drop-layer-kind`.

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
#[dsl(keyword = "drop-layer-kind")]
pub struct DropLayerKind {
    pub kind: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the newly-dropped layer used to also
/// select itself here — the `"layers"` domain's selection is framework-owned `InteractionState` now,
/// only ever mutated by the framework's own injected `interactionSelect` handling, never by an app
/// command's `Emit::config_mutations`.
pub fn handle(payload: &DropLayerKind, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = create_layer_of_kind(&payload.kind);
    Ok(Emit {
        artifact_mutations: vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index: document.layers.len(), layer: Box::new(layer) })],
        ..Default::default()
    })
}
