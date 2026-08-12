//! ↩️ `change-layer-visible` inverse — the old `visible` value from `base`; `change` is its own
//! inverse partner. Missing target ⇒ `Vec::new()`.

use crate::artifacts::raster::schema::{find_layer, layer_visible};
use crate::artifacts::raster::mutations::change_layer_visible::mutation::ChangeLayerVisible;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeLayerVisible, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![RasterMutation::ChangeLayerVisible(ChangeLayerVisible { layer_id: payload.layer_id.clone(), new_visible: layer_visible(layer) })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
