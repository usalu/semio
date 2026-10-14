//! 🔺️ `delete-layer` sparse diff — delegates to the existing `diff_remove_layer` helper (its logic
//! already matches: a single `layers.removed` row at the layer's base address, cascade handled at apply-time by
//! `remove_layer_from_tree` recursing into the whole removed subtree).

use semio_s_artifact_raster_raster_diff::{diff_remove_layer, RasterDiff};
use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_layer::mutation::DeleteLayer, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if find_layer(&base.layers, &payload.layer_id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_remove_layer(&base.layers, &payload.layer_id))
}
//#endregion 🔖️Diff
