//! ↩️ Inverse for `SetLayerBooleanOperation` — the OLD operation captured from BASE. Missing target
//! or a non-boolean layer ⇒ `Vec::new()`.
use crate::artifacts::draw::engine::find_draw_layer;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetLayerBooleanOperation, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(DrawLayerNode::Boolean(boolean)) => vec![super::mutation::set_layer_boolean_operation(payload.layer_id.clone(), boolean.operation.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
