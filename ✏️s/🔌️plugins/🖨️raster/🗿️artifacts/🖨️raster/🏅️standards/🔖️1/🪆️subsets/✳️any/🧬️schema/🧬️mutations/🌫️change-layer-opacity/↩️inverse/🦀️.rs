//! ↩️ `change-layer-opacity` inverse — the old `opacity` value from `base`. Missing target ⇒
//! `Vec::new()`.


use semio_s_artifact_raster_raster_model::{find_layer, layer_opacity};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_layer_opacity::mutation::ChangeLayerOpacity>>(payload: &crate::change_layer_opacity::mutation::ChangeLayerOpacity, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![Op::from(crate::change_layer_opacity::mutation::ChangeLayerOpacity { layer_id: payload.layer_id.clone(), new_opacity: layer_opacity(layer) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
