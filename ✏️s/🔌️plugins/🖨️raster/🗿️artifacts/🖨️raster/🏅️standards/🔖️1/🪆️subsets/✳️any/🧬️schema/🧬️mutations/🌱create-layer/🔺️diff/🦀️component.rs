//! 🔺️ `create-layer` sparse diff — a tree-aware insertion, never a whole-snapshot capture.

use crate::artifacts::raster::diff::{diff_add_layer, RasterDiff};
use crate::artifacts::raster::mutations::create_layer::mutation::CreateLayer;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateLayer, _base: &RasterSnapshot) -> RasterDiff {
    diff_add_layer(payload.parent_id.clone(), payload.index, (*payload.layer).clone())
}
//#endregion 🔖️Diff
