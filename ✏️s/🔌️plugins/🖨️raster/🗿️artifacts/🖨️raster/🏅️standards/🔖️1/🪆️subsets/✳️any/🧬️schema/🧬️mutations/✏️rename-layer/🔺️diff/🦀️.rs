//! 🔺️ `rename-layer` sparse diff — writes only the layer's `name` via the existing `diff_patch_layer`
//! helper (the `RasterLayerPatch` here is a diff-internal type only, never the mutation's own payload).

use semio_s_artifact_raster_raster_diff::{diff_patch_layer, RasterDiff};
use semio_s_artifact_raster_raster_model::{find_layer, layer_name};
use semio_s_artifact_raster_raster_model::RasterLayerPatch;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::rename_layer::mutation::RenameLayer, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    let Some(layer) = find_layer(&base.layers, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_name(layer) == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" is already named \"{}\".", payload.layer_id, payload.new_name));
    }
    protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { name: Some(payload.new_name.clone()), ..Default::default() }))
}
//#endregion 🔖️Diff
