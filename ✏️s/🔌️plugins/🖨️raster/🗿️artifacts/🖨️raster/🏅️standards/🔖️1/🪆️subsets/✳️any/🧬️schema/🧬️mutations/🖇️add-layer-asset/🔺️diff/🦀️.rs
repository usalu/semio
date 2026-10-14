//! 🔺️ `add-layer-asset` sparse diff — a single `assets.entries` insertion.

use semio_s_artifact_raster_raster_diff::{diff_add_asset, RasterDiff};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::add_layer_asset::mutation::AddLayerAsset, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if base.assets.contains_key(&payload.asset_id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Asset \"{}\" is already attached.", payload.asset_id));
    }
    protocol::MutationOutcome::new(diff_add_asset(&payload.asset_id, payload.asset.clone()))
}
//#endregion 🔖️Diff
