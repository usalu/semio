//! ↩️ `remove-layer-asset` inverse — captures the removed asset's bytes from `base`. Missing target
//! ⇒ `Vec::new()`.

use crate::artifacts::raster::mutations::add_layer_asset;
use crate::artifacts::raster::mutations::remove_layer_asset::mutation::RemoveLayerAsset;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveLayerAsset, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match base.assets.get(&payload.asset_id) {
        Some(asset) => vec![RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: payload.asset_id.clone(), asset: asset.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
