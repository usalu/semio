//! ↩️ `move-layer` inverse — the old `transform.x`/`.y` from `base`; `move` is its own inverse
//! partner. Missing target ⇒ `Vec::new()`.

use crate::mutations::RasterMutation;
use crate::standards::v1::subsets::any::schema::{find_layer, layer_transform};
use crate::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveLayer, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => {
            let transform = layer_transform(layer);
            vec![RasterMutation::MoveLayer(super::MoveLayer { layer_id: payload.layer_id.clone(), new_x: transform.x, new_y: transform.y })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
