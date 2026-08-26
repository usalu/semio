//! 🔺️ `remove-layer-asset` sparse diff — a single `assets.entries` removal.

use crate::artifacts::raster::diff::{diff_remove_asset, RasterDiff};
use crate::artifacts::raster::mutations::remove_layer_asset::mutation::RemoveLayerAsset;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveLayerAsset, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if !base.assets.contains_key(&payload.asset_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.asset_id), [payload.asset_id.clone()]);
    }
    protocol::MutationOutcome::new(diff_remove_asset(&payload.asset_id))
}
//#endregion 🔖️Diff
