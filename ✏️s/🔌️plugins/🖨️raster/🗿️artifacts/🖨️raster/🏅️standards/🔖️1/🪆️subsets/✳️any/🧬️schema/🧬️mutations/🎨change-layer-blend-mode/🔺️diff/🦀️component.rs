//! 🔺️ `change-layer-blend-mode` sparse diff — writes only the layer's `blend_mode` field.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::change_layer_blend_mode::mutation::ChangeLayerBlendMode;
use crate::artifacts::raster::{RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerBlendMode, _base: &RasterSnapshot) -> RasterDiff {
    diff_patch_layer(&payload.layer_id, RasterLayerPatch { blend_mode: Some(payload.new_blend_mode.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
