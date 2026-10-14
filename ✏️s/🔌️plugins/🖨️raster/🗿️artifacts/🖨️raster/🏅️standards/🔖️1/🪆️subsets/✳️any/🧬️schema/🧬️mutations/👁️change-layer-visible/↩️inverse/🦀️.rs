//! ↩️ `change-layer-visible` inverse — the old `visible` value from `base`; `change` is its own
//! inverse partner. Missing target ⇒ `Vec::new()`.


use semio_s_artifact_raster_raster_model::{find_layer, layer_visible};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_layer_visible::mutation::ChangeLayerVisible>>(payload: &crate::change_layer_visible::mutation::ChangeLayerVisible, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![Op::from(crate::change_layer_visible::mutation::ChangeLayerVisible { layer_id: payload.layer_id.clone(), new_visible: layer_visible(layer) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
