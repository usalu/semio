//! 🔺️ `change-layer-visible` sparse diff — writes only the layer's `visible` field.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::change_layer_visible::mutation::ChangeLayerVisible;
use crate::artifacts::raster::{RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerVisible, _base: &RasterSnapshot) -> RasterDiff {
    diff_patch_layer(&payload.layer_id, RasterLayerPatch { visible: Some(payload.new_visible), ..Default::default() })
}
//#endregion 🔖️Diff
