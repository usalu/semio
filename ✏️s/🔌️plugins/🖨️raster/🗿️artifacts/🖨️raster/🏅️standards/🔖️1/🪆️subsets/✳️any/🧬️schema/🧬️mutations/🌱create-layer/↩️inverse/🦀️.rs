//! ↩️ `create-layer` inverse — `delete-layer` addressed by the created layer's own id (no `base`
//! lookup needed, the id is already on the payload).

use crate::artifacts::raster::mutations::{delete_layer, RasterMutation};
use crate::artifacts::raster::schema::layer_node_id;
use crate::artifacts::raster::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateLayer, _base: &RasterSnapshot) -> Vec<RasterMutation> {
    vec![RasterMutation::DeleteLayer(delete_layer::DeleteLayer { layer_id: layer_node_id(&payload.layer).to_string() })]
}
//#endregion 🔖️Inverse
