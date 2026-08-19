//! ↩️ `move-layer` inverse — the old `transform.x`/`.y` from `base`; `move` is its own inverse
//! partner. Missing target ⇒ `Vec::new()`.

use crate::artifacts::raster::schema::{find_layer, layer_transform};
use crate::artifacts::raster::mutations::move_layer::mutation::MoveLayer;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &MoveLayer, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => {
            let transform = layer_transform(layer);
            vec![RasterMutation::MoveLayer(MoveLayer { layer_id: payload.layer_id.clone(), new_x: transform.x, new_y: transform.y })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
