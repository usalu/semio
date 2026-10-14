//! ↩️ `move-layer` inverse — the old `transform.x`/`.y` from `base`; `move` is its own inverse
//! partner. Missing target ⇒ `Vec::new()`.


use semio_s_artifact_raster_raster_model::{find_layer, layer_transform};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::move_layer::mutation::MoveLayer>>(payload: &crate::move_layer::mutation::MoveLayer, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => {
            let transform = layer_transform(layer);
            vec![Op::from(crate::move_layer::mutation::MoveLayer { layer_id: payload.layer_id.clone(), new_x: transform.x, new_y: transform.y })]
        }
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
