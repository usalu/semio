//! ↩️ `delete-layer` inverse — captures the FULL removed subtree (`find_layer` clones the whole
//! matched node, children included for a `Group`) plus its tree address from `base`, and re-`create`s
//! it there. Missing target ⇒ `Vec::new()`.

use crate::artifacts::raster::mutations::create_layer;
use crate::artifacts::raster::mutations::delete_layer::mutation::DeleteLayer;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::{find_layer, locate_layer};
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteLayer, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match (locate_layer(&base.layers, &payload.layer_id), find_layer(&base.layers, &payload.layer_id)) {
        (Some((parent_id, index)), Some(layer)) => vec![RasterMutation::CreateLayer(create_layer::mutation::CreateLayer { parent_id, index, layer: Box::new(layer.clone()) })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
