//! ↩️ `change-layer-blend-mode` inverse — the old `blend_mode` value from `base`. Missing target ⇒
//! `Vec::new()`.

use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, layer_blend_mode};
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeLayerBlendMode, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![RasterMutation::ChangeLayerBlendMode(super::ChangeLayerBlendMode { layer_id: payload.layer_id.clone(), new_blend_mode: layer_blend_mode(layer).to_string() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
