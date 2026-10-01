//! ↩️ Inverse for `SetLayerBooleanOperation` — the OLD operation captured from BASE. Missing target
//! or a non-boolean layer ⇒ `Vec::new()`.
use crate::mutations::DrawingMutation;
use crate::schema::find_drawing_layer;
use crate::{DrawingLayerNode, DrawingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetLayerBooleanOperation, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(DrawingLayerNode::Boolean(boolean)) => vec![super::mutation::set_layer_boolean_operation(payload.layer_id.clone(), boolean.operation.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
