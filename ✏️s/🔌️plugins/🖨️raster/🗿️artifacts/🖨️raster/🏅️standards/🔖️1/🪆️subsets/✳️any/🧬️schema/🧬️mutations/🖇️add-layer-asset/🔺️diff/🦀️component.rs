//! 🔺️ `add-layer-asset` sparse diff — a single `assets.entries` insertion.

use crate::artifacts::raster::diff::{diff_add_asset, RasterDiff};
use crate::artifacts::raster::mutations::add_layer_asset::mutation::AddLayerAsset;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &AddLayerAsset, _base: &RasterSnapshot) -> RasterDiff {
    diff_add_asset(&payload.asset_id, payload.asset.clone())
}
//#endregion 🔖️Diff
