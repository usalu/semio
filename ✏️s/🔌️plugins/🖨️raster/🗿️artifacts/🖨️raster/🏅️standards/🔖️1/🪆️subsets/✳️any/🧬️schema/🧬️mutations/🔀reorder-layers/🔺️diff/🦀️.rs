//! 🔺️ `reorder-layers` sparse diff — a tree-aware remove-then-insert move, delegating to
//! `diff_move_layer` (fixed to be genuinely sparse: no clone-mutate-diff of the whole snapshot).

use crate::artifacts::raster::diff::{diff_move_layer, RasterDiff};
use crate::artifacts::raster::schema::{find_layer, layer_node_id};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};

//#region 🔖️Diff
/// 📐️ Finds `target_id`'s current `(parent_id, index)` address, recursing into groups.
fn locate(layers: &[RasterLayerNode], parent_id: Option<&str>, target_id: &str) -> Option<(Option<String>, usize)> {
    for (index, layer) in layers.iter().enumerate() {
        if layer_node_id(layer) == target_id {
            return Some((parent_id.map(str::to_string), index));
        }
        if let RasterLayerNode::Group { id, children, .. } = layer {
            if let Some(found) = locate(children, Some(id.as_str()), target_id) {
                return Some(found);
            }
        }
    }
    None
}

pub fn diff(payload: &super::ReorderLayers, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if find_layer(&base.layers, &payload.layer_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    }
    if let Some(parent_id) = payload.parent_id.as_deref() {
        if find_layer(&base.layers, parent_id).is_none() {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Parent layer \"{}\" does not exist.", parent_id), [parent_id.to_string()]);
        }
    }
    if let Some((current_parent, current_index)) = locate(&base.layers, None, &payload.layer_id) {
        if current_parent == payload.parent_id && current_index == payload.index {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" is already at that position.", payload.layer_id));
        }
    }
    protocol::MutationOutcome::new(diff_move_layer(&payload.layer_id, payload.parent_id.clone(), payload.index))
}
//#endregion 🔖️Diff
