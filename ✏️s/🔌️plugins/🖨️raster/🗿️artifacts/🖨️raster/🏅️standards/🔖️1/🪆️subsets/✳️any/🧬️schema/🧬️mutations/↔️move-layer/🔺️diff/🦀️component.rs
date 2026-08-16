//! 🔺️ `move-layer` sparse diff — writes only the layer's `transform.x`/`.y`.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::move_layer::mutation::MoveLayer;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveLayer, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    let Some(layer) = find_layer(&base.layers, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" position must be finite, got ({}, {}).", payload.layer_id, payload.new_x, payload.new_y), [payload.layer_id.clone()]);
    }
    let (x, y) = match layer {
        RasterLayerNode::Pixel { transform, .. } | RasterLayerNode::Group { transform, .. } | RasterLayerNode::Adjustment { transform, .. } => (transform.x, transform.y),
    };
    if x == payload.new_x && y == payload.new_y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" is already at ({}, {}).", payload.layer_id, payload.new_x, payload.new_y));
    }
    protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { transform_x: Some(payload.new_x), transform_y: Some(payload.new_y), ..Default::default() }))
}
//#endregion 🔖️Diff
