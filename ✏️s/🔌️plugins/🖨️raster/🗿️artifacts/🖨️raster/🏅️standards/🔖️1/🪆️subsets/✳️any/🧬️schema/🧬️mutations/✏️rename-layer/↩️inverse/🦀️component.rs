//! ↩️ `rename-layer` inverse — the old name from `base`; `rename` is its own inverse partner (per
//! `📓️taxonomy.md`). Missing target ⇒ `Vec::new()`.

use crate::artifacts::raster::mutations::rename_layer::mutation::RenameLayer;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_name};
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RenameLayer, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![RasterMutation::RenameLayer(RenameLayer { layer_id: payload.layer_id.clone(), new_name: layer_name(layer).to_string() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
