//! ↩️ Inverse for `ReorderLayer` — the OLD `(parent_id, index)` address captured from BASE. Missing
//! target ⇒ `Vec::new()`.

use semio_s_artifact_draw_drawing_model::find_drawing_layer_location;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::reorder_layer::mutation::ReorderLayer>>(payload: &super::mutation::ReorderLayer, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer_location(base, &payload.layer_id) {
        Some(location) => vec![super::mutation::reorder_layer(payload.layer_id.clone(), location.parent_id, location.index)],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
