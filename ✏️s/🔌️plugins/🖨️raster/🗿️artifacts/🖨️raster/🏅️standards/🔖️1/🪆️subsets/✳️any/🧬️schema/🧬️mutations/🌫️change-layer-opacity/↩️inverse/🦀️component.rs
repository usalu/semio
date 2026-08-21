//! ↩️ `change-layer-opacity` inverse — the old `opacity` value from `base`. Missing target ⇒
//! `Vec::new()`.

use crate::artifacts::raster::mutations::change_layer_opacity::mutation::ChangeLayerOpacity;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_opacity};
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ChangeLayerOpacity, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![RasterMutation::ChangeLayerOpacity(ChangeLayerOpacity { layer_id: payload.layer_id.clone(), new_opacity: layer_opacity(layer) })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
