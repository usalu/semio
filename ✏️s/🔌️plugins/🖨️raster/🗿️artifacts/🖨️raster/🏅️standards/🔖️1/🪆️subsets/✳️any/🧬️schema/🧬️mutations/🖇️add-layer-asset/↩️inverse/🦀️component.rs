//! ↩️ `add-layer-asset` inverse — `remove-layer-asset` addressed by the added asset's own id (no
//! `base` lookup needed, the id is already on the payload).

use crate::artifacts::raster::mutations::add_layer_asset::mutation::AddLayerAsset;
use crate::artifacts::raster::mutations::remove_layer_asset;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &AddLayerAsset, _base: &RasterSnapshot) -> Vec<RasterMutation> {
    vec![RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: payload.asset_id.clone() })]
}
//#endregion 🔖️Inverse
