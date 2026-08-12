//! 🔺️ `remove-layer-asset` sparse diff — a single `assets.entries` removal.

use crate::artifacts::raster::diff::{diff_remove_asset, RasterDiff};
use crate::artifacts::raster::mutations::remove_layer_asset::mutation::RemoveLayerAsset;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveLayerAsset, _base: &RasterSnapshot) -> RasterDiff {
    diff_remove_asset(&payload.asset_id)
}
//#endregion 🔖️Diff
