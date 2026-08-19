//! ↩️ `add-layer-asset` inverse — computed from `base`, because `assets` is a map and an `add` over
//! an already-present key is an OVERWRITE, not an insert: undoing it means re-adding the PRIOR
//! asset, not removing the key. Only a genuinely new key inverts to `remove-layer-asset`.

use crate::artifacts::raster::mutations::add_layer_asset::mutation::AddLayerAsset;
use crate::artifacts::raster::mutations::remove_layer_asset;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
/// 🩹️ `base.assets` now stores a handle, not bytes — the real prior asset content is recovered
/// through `crate::artifacts::raster::raster_asset` (the working-scene cache accessor, ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`). A cold cache (documented staleness gap: the cache
/// is in-process only) is distinguished from a genuinely-new key by checking handle presence FIRST —
/// a present-handle-but-cold-cache fails soft to a no-op inverse (never the destructive
/// `RemoveLayerAsset` a naive "content missing ⇒ treat as new" read would wrongly emit).
pub async fn inverse(payload: &AddLayerAsset, base: &RasterSnapshot) -> Vec<RasterMutation> {
    if base.assets.get(&payload.asset_id).is_none() {
        return vec![RasterMutation::RemoveLayerAsset(remove_layer_asset::mutation::RemoveLayerAsset { asset_id: payload.asset_id.clone() })];
    }
    match crate::artifacts::raster::raster_asset(&base.assets, &payload.asset_id) {
        Some(prior) => vec![RasterMutation::AddLayerAsset(AddLayerAsset { asset_id: payload.asset_id.clone(), asset: prior })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
