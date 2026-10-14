//! ↩️ Inverse for `SetLayerBooleanOperation` — the OLD operation captured from BASE. Missing target
//! or a non-boolean layer ⇒ `Vec::new()`.

use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::set_layer_boolean_operation::mutation::SetLayerBooleanOperation>>(payload: &super::mutation::SetLayerBooleanOperation, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(DrawingLayerNode::Boolean(boolean)) => vec![super::mutation::set_layer_boolean_operation(payload.layer_id.clone(), boolean.operation.clone())],
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
