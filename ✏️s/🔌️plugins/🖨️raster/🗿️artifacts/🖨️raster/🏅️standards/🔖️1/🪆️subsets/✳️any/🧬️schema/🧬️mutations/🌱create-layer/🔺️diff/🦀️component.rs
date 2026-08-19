//! 🔺️ `create-layer` sparse diff — a tree-aware insertion, never a whole-snapshot capture.

use crate::artifacts::raster::diff::{diff_add_layer, RasterDiff};
use crate::artifacts::raster::mutations::create_layer::mutation::CreateLayer;
use crate::artifacts::raster::schema::{find_layer, layer_node_id};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateLayer, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    let new_id = layer_node_id(&payload.layer);
    if find_layer(&base.layers, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A layer with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    if let Some(parent_id) = payload.parent_id.as_deref() {
        match find_layer(&base.layers, parent_id) {
            Some(RasterLayerNode::Group { .. }) => {}
            _ => return protocol::MutationOutcome::fatal("mutation.invariant", format!("Parent layer \"{}\" does not exist or is not a group.", parent_id), [parent_id.to_string()]),
        }
    }
    protocol::MutationOutcome::new(diff_add_layer(payload.parent_id.clone(), payload.index, (*payload.layer).clone()))
}
//#endregion 🔖️Diff
