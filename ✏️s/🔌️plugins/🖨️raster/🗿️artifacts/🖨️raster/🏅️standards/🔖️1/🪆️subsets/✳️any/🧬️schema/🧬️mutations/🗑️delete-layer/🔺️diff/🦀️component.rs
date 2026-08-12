//! 🔺️ `delete-layer` sparse diff — delegates to the existing `diff_remove_layer` helper (its logic
//! already matches: a single `layers.removed` entry, cascade handled at apply-time by
//! `remove_layer_from_tree` recursing into the whole removed subtree).

use crate::artifacts::raster::diff::{diff_remove_layer, RasterDiff};
use crate::artifacts::raster::mutations::delete_layer::mutation::DeleteLayer;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLayer, _base: &RasterSnapshot) -> RasterDiff {
    diff_remove_layer(&payload.layer_id)
}
//#endregion 🔖️Diff
