//! 🔺️ `delete-layer` sparse diff — delegates to the existing `diff_remove_layer` helper (its logic
//! already matches: a single `layers.removed` entry, cascade handled at apply-time by
//! `remove_layer_from_tree` recursing into the whole removed subtree).

use crate::artifacts::raster::diff::{diff_remove_layer, RasterDiff};
use crate::artifacts::raster::mutations::delete_layer::mutation::DeleteLayer;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteLayer, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if find_layer(&base.layers, &payload.layer_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_remove_layer(&payload.layer_id))
}
//#endregion 🔖️Diff
