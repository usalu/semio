//! ↩️ Inverse for `ReplaceLayerFill` — the OLD fill payload captured from BASE.

use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::replace_layer_fill::mutation::ReplaceLayerFill>>(payload: &super::mutation::ReplaceLayerFill, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::replace_layer_fill(payload.layer_id.clone(), layer_base(layer).attributes.fill.clone())],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
