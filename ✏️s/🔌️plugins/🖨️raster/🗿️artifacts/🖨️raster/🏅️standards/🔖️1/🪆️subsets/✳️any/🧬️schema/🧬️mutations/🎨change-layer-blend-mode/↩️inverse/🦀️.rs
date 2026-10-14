//! ↩️ `change-layer-blend-mode` inverse — the old `blend_mode` value from `base`. Missing target ⇒
//! `Vec::new()`.


use semio_s_artifact_raster_raster_model::{find_layer, layer_blend_mode};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_layer_blend_mode::mutation::ChangeLayerBlendMode>>(payload: &crate::change_layer_blend_mode::mutation::ChangeLayerBlendMode, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![Op::from(crate::change_layer_blend_mode::mutation::ChangeLayerBlendMode { layer_id: payload.layer_id.clone(), new_blend_mode: layer_blend_mode(layer).to_string() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
