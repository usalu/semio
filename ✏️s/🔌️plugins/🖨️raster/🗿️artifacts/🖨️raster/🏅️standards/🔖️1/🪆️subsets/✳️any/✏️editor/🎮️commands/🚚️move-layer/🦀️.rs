//! 🖼️ 🖼️ Raster play app commands command — `move-layer`.

use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::mutations::reorder_layers;
use crate::op::RasterMutation;
use crate::standards::v1::subsets::any::schema::{find_layer, locate_layer};
use crate::{RasterLayerNode, RasterSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "move-layer")]
pub struct MoveLayer {
    pub layer_id: String,
    pub target_row_id: String,
    pub drop_position: String,
}

/// 🌳️ Resolve a tree drop to an index in the destination after removing the source.
fn resolve_drop(payload: &MoveLayer, document: &RasterSnapshot) -> Result<Option<reorder_layers::mutation::ReorderLayers>, Fault> {
    if !matches!(payload.drop_position.as_str(), "before" | "after" | "inside") { return Err(Fault::from("raster-layer-drop-position-invalid")); }
    let source = find_layer(&document.layers, &payload.layer_id).ok_or_else(|| Fault::from("raster-layer-source-missing"))?;
    let target = find_layer(&document.layers, &payload.target_row_id).ok_or_else(|| Fault::from("raster-layer-target-missing"))?;
    if payload.layer_id == payload.target_row_id { return Ok(None); }
    if let RasterLayerNode::Group { children, .. } = source {
        if find_layer(children, &payload.target_row_id).is_some() { return Err(Fault::from("raster-layer-drop-cycle")); }
    }
    let (source_parent, source_index) = locate_layer(&document.layers, &payload.layer_id).unwrap();
    let (parent_id, mut index) = if payload.drop_position == "inside" {
        let RasterLayerNode::Group { children, .. } = target else { return Err(Fault::from("raster-layer-drop-requires-group")); };
        (Some(payload.target_row_id.clone()), children.len())
    } else {
        let (parent, index) = locate_layer(&document.layers, &payload.target_row_id).unwrap();
        (parent, index + usize::from(payload.drop_position == "after"))
    };
    if source_parent == parent_id && source_index < index { index -= 1; }
    if source_parent == parent_id && source_index == index { return Ok(None); }
    Ok(Some(reorder_layers::mutation::ReorderLayers { layer_id: payload.layer_id.clone(), parent_id, index }))
}

pub fn handle(payload: &MoveLayer, doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Ok(match resolve_drop(payload, doc.snapshot)? {
        Some(change) => Emit::mutations(vec![RasterMutation::ReorderLayers(change)]),
        None => Emit::default(),
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
