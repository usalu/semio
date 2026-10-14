//! ↩️ `delete-layer` inverse — captures the FULL removed subtree (`find_layer` clones the whole
//! matched node, children included for a `Group`) plus its tree address from `base`, and re-`create`s
//! it there. Missing target ⇒ `Vec::new()`.

use crate::create_layer;
use semio_s_artifact_raster_raster_model::{find_layer, locate_layer};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::create_layer::mutation::CreateLayer>>(payload: &crate::delete_layer::mutation::DeleteLayer, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match (locate_layer(&base.layers, &payload.layer_id), find_layer(&base.layers, &payload.layer_id)) {
        (Some((parent_id, index)), Some(layer)) => vec![Op::from(create_layer::CreateLayer { parent_id, index, layer: Box::new(layer.clone()) })],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
