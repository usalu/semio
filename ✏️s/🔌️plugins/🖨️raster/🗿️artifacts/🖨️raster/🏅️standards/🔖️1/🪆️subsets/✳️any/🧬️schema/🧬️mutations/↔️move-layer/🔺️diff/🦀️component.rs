//! 🔺️ `move-layer` sparse diff — writes only the layer's `transform.x`/`.y`.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::move_layer::mutation::MoveLayer;
use crate::artifacts::raster::{RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &MoveLayer, _base: &RasterSnapshot) -> RasterDiff {
    diff_patch_layer(&payload.layer_id, RasterLayerPatch { transform_x: Some(payload.new_x), transform_y: Some(payload.new_y), ..Default::default() })
}
//#endregion 🔖️Diff
