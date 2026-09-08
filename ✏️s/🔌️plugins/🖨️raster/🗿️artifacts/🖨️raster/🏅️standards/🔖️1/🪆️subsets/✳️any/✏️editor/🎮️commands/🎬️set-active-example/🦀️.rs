//! 🎬️ 🖼️ Raster play app commands command — `set-active-example`.

use crate::mutations::{add_layer_asset, create_layer, delete_layer, remove_layer_asset};
use crate::op::RasterMutation;
use crate::schema::{layer_node_id, raster_example_document};
use crate::{raster_asset, RasterSnapshot};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
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
fn replace_document_operations(current: &RasterSnapshot, next: &RasterSnapshot) -> Vec<RasterMutation> {
    let mut operations = Vec::new();
    for layer in &current.layers {
        operations.push(RasterMutation::DeleteLayer(delete_layer::mutation::DeleteLayer { layer_id: layer_node_id(layer).to_string() }));
    }
    for asset_id in current.assets.keys() {
        operations.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: asset_id.clone() }));
    }
    for (asset_id, _) in next.assets.iter() {
        if let Some(asset) = raster_asset(&next.assets, asset_id) {
            operations.push(RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: asset_id.clone(), asset }));
        }
    }
    for (index, layer) in next.layers.iter().enumerate() {
        operations.push(RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id: None, index, layer: Box::new(layer.clone()) }));
    }
    operations
}

/// 🎬️ Loads one registered example over the open document. An id this subset does not register is a
/// no-op rather than a fault — the navbar switcher is free-text on the wire, and an unknown id must
/// not destroy the open document.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    let Some(example) = raster_example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };
    Ok(Emit::mutations(replace_document_operations(doc.snapshot, &example)))
}
