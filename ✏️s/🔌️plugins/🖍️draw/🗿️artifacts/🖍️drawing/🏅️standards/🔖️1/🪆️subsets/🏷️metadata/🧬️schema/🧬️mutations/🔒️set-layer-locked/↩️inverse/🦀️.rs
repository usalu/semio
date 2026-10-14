//! ↩️ Inverse for `SetLayerLocked` — reconstructed from BASE state.

use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::set_layer_locked::mutation::SetLayerLocked>>(payload: &super::mutation::SetLayerLocked, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_locked(payload.layer_id.clone(), layer_base(layer).locked)],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
