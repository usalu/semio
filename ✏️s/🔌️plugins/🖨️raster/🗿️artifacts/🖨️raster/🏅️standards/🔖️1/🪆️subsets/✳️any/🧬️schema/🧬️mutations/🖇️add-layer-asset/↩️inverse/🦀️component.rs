//! ↩️ `add-layer-asset` inverse — computed from `base`, because `assets` is a map and an `add` over
//! an already-present key is an OVERWRITE, not an insert: undoing it means re-adding the PRIOR
//! asset, not removing the key. Only a genuinely new key inverts to `remove-layer-asset`.

use crate::artifacts::raster::mutations::add_layer_asset::mutation::AddLayerAsset;
use crate::artifacts::raster::mutations::remove_layer_asset;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &AddLayerAsset, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match base.assets.get(&payload.asset_id) {
        Some(prior) => vec![RasterMutation::AddLayerAsset(AddLayerAsset { asset_id: payload.asset_id.clone(), asset: prior.clone() })],
        None => vec![RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: payload.asset_id.clone() })],
    }
}
//#endregion 🔖️Inverse
