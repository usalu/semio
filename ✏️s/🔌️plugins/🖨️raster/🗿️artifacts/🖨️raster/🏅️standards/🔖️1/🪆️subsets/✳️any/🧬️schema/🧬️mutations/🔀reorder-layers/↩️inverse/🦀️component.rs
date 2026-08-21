//! ↩️ `reorder-layers` inverse — the layer's pre-move tree address from `base`. Missing target ⇒
//! `Vec::new()`.

use crate::artifacts::raster::mutations::reorder_layers::mutation::ReorderLayers;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::locate_layer;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReorderLayers, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match locate_layer(&base.layers, &payload.layer_id) {
        Some((parent_id, index)) => vec![RasterMutation::ReorderLayers(ReorderLayers { layer_id: payload.layer_id.clone(), parent_id, index })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
