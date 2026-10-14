//! 🔺️ `remove-layer-asset` sparse diff — a single `assets.entries` removal.

use semio_s_artifact_raster_raster_diff::{diff_remove_asset, RasterDiff};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_layer_asset::mutation::RemoveLayerAsset, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if !base.assets.contains_key(&payload.asset_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.asset_id), [payload.asset_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_remove_asset(&payload.asset_id))
}
//#endregion 🔖️Diff
