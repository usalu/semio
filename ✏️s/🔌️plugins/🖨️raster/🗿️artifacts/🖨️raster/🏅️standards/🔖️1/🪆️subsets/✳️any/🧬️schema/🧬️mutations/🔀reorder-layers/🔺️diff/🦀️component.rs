//! 🔺️ `reorder-layers` sparse diff — a tree-aware remove-then-insert move, delegating to
//! `diff_move_layer` (fixed to be genuinely sparse: no clone-mutate-diff of the whole snapshot).

use crate::artifacts::raster::diff::{diff_move_layer, RasterDiff};
use crate::artifacts::raster::mutations::reorder_layers::mutation::ReorderLayers;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReorderLayers, _base: &RasterSnapshot) -> RasterDiff {
    diff_move_layer(&payload.layer_id, payload.parent_id.clone(), payload.index)
}
//#endregion 🔖️Diff
