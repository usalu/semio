//! 🔺️ `add-layer-asset` sparse diff — a single `assets.entries` insertion.

use crate::artifacts::raster::diff::{diff_add_asset, RasterDiff};
use crate::artifacts::raster::mutations::add_layer_asset::mutation::AddLayerAsset;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &AddLayerAsset, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if base.assets.contains_key(&payload.asset_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Asset \"{}\" is already attached.", payload.asset_id));
    }
    protocol::MutationOutcome::new(diff_add_asset(&payload.asset_id, payload.asset.clone()))
}
//#endregion 🔖️Diff
