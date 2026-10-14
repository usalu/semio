//! ↩️ Inverse for `SetLayerBlendMode` — reconstructed from BASE state.

use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::set_layer_blend_mode::mutation::SetLayerBlendMode>>(payload: &super::mutation::SetLayerBlendMode, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_blend_mode(payload.layer_id.clone(), layer_base(layer).blend_mode.clone())],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
