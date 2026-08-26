//! ↩️ `remove-layer-asset` inverse — captures the removed asset's real bytes from `base` via the
//! working-scene cache accessor (`crate::artifacts::raster::raster_asset`, ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `base.assets` now stores a handle, not bytes).
//! Missing target OR a cold cache (documented staleness gap) ⇒ `Vec::new()` — both fail soft to a
//! no-op inverse, never a panic.

use crate::artifacts::raster::mutations::add_layer_asset;
use crate::artifacts::raster::mutations::remove_layer_asset::mutation::RemoveLayerAsset;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveLayerAsset, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match crate::artifacts::raster::raster_asset(&base.assets, &payload.asset_id) {
        Some(asset) => vec![RasterMutation::AddLayerAsset(add_layer_asset::mutation::AddLayerAsset { asset_id: payload.asset_id.clone(), asset })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
