//! ↩️ Inverse for `RenameLayer` — the OLD name looked up from BASE, never a captured id.

use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::rename_layer::mutation::RenameLayer>>(payload: &super::mutation::RenameLayer, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::rename_layer(payload.layer_id.clone(), layer_base(layer).name.clone())],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
