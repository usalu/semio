//! ↩️ `resize-layer` inverse — the old `width`/`height` from `base` (defaulting like
//! `apply_layer_patch` does when unset). Not a `Pixel`, or missing target ⇒ `Vec::new()`.

use crate::artifacts::raster::engine::find_layer;
use crate::artifacts::raster::mutations::resize_layer::mutation::ResizeLayer;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ResizeLayer, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Pixel { width, height, .. }) => {
            vec![RasterMutation::ResizeLayer(ResizeLayer { layer_id: payload.layer_id.clone(), new_width: width.unwrap_or(512), new_height: height.unwrap_or(512) })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
