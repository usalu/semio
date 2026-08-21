//! 🔺️ `resize-layer` sparse diff — writes only the layer's `width`/`height`; `RasterDiff::default()`
//! when the addressed layer isn't a `Pixel` (or doesn't exist).

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::resize_layer::mutation::ResizeLayer;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ResizeLayer, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    match find_layer(&base.layers, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]),
        Some(_) if payload.new_width == 0 || payload.new_height == 0 => {
            protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" size must be positive, got {}x{}.", payload.layer_id, payload.new_width, payload.new_height), [payload.layer_id.clone()])
        }
        Some(RasterLayerNode::Pixel { width, height, .. }) if *width == Some(payload.new_width) && *height == Some(payload.new_height) => {
            protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" is already {}x{}.", payload.layer_id, payload.new_width, payload.new_height))
        }
        Some(RasterLayerNode::Pixel { .. }) => protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { width: Some(payload.new_width), height: Some(payload.new_height), ..Default::default() })),
        Some(_) => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" is not a pixel layer.", payload.layer_id), [payload.layer_id.clone()]),
    }
}
//#endregion 🔖️Diff
