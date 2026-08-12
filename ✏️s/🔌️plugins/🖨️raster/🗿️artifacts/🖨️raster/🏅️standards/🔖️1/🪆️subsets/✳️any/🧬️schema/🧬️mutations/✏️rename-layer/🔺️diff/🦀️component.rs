//! 🔺️ `rename-layer` sparse diff — writes only the layer's `name` via the existing `diff_patch_layer`
//! helper (the `RasterLayerPatch` here is a diff-internal type only, never the mutation's own payload).

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::rename_layer::mutation::RenameLayer;
use crate::artifacts::raster::{RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameLayer, _base: &RasterSnapshot) -> RasterDiff {
    diff_patch_layer(&payload.layer_id, RasterLayerPatch { name: Some(payload.new_name.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
