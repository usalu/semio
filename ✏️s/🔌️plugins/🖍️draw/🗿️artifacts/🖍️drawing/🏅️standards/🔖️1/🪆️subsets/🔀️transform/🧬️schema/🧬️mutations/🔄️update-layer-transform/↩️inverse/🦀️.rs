//! ↩️ Inverse for `UpdateLayerTransform` — the OLD transform captured from BASE.

use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::update_layer_transform::mutation::UpdateLayerTransform>>(payload: &super::mutation::UpdateLayerTransform, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::update_layer_transform(payload.layer_id.clone(), layer_base(layer).transform.clone())],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
