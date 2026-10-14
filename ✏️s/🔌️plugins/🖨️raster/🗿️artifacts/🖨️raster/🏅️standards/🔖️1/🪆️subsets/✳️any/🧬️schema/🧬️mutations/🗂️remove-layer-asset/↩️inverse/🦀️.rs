//! ↩️ `remove-layer-asset` inverse — captures the removed asset's real bytes from `base` via the
//! working-scene cache accessor (`crate::raster_asset`, ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `base.assets` now stores a handle, not bytes).
//! Missing target OR a cold cache (documented staleness gap) ⇒ `Vec::new()` — both fail soft to a
//! no-op inverse, never a panic.

use crate::add_layer_asset;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::add_layer_asset::mutation::AddLayerAsset>>(payload: &crate::remove_layer_asset::mutation::RemoveLayerAsset, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match semio_s_artifact_raster_raster_model::raster_image(&base.assets, &payload.asset_id) {
        Some(asset) => vec![Op::from(add_layer_asset::AddLayerAsset { asset_id: payload.asset_id.clone(), asset })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
