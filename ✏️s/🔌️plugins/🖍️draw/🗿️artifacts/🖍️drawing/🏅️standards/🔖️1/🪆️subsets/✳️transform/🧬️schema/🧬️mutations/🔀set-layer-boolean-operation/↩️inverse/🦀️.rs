//! ↩️ Inverse for `SetLayerBooleanOperation` — the OLD operation captured from BASE. Missing target
//! or a non-boolean layer ⇒ `Vec::new()`.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::find_drawing_layer;
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetLayerBooleanOperation, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(DrawingLayerNode::Boolean(boolean)) => vec![super::mutation::set_layer_boolean_operation(payload.layer_id.clone(), boolean.operation.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
