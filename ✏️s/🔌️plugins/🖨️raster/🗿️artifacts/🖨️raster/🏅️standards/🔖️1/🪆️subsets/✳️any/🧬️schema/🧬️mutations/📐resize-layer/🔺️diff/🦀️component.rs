//! 🔺️ `resize-layer` sparse diff — writes only the layer's `width`/`height`; `RasterDiff::default()`
//! when the addressed layer isn't a `Pixel` (or doesn't exist).

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::engine::find_layer;
use crate::artifacts::raster::mutations::resize_layer::mutation::ResizeLayer;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ResizeLayer, base: &RasterSnapshot) -> RasterDiff {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Pixel { .. }) => diff_patch_layer(&payload.layer_id, RasterLayerPatch { width: Some(payload.new_width), height: Some(payload.new_height), ..Default::default() }),
        _ => RasterDiff::default(),
    }
}
//#endregion 🔖️Diff
