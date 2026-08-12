//! 🔺️ `change-layer-opacity` sparse diff — writes only the layer's `opacity` field.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::change_layer_opacity::mutation::ChangeLayerOpacity;
use crate::artifacts::raster::{RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerOpacity, _base: &RasterSnapshot) -> RasterDiff {
    diff_patch_layer(&payload.layer_id, RasterLayerPatch { opacity: Some(payload.new_opacity), ..Default::default() })
}
//#endregion 🔖️Diff
