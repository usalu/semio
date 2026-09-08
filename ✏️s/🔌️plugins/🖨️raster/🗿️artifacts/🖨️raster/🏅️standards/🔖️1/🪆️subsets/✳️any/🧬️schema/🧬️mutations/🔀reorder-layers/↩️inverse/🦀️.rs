//! ↩️ `reorder-layers` inverse — the layer's pre-move tree address from `base`. Missing target ⇒
//! `Vec::new()`.

use crate::mutations::RasterMutation;
use crate::schema::locate_layer;
use crate::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReorderLayers, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match locate_layer(&base.layers, &payload.layer_id) {
        Some((parent_id, index)) => vec![RasterMutation::ReorderLayers(super::ReorderLayers { layer_id: payload.layer_id.clone(), parent_id, index })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
