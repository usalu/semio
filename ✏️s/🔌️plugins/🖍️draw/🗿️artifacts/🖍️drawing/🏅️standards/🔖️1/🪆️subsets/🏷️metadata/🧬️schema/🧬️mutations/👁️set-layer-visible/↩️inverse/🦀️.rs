//! ↩️ Inverse for `SetLayerVisible` — real logic reconstructing the undo from BASE state, never
//! from post-state. Missing target ⇒ `Vec::new()` (the semantic replacement for `NoMutation`).
use crate::mutations::DrawingMutation;
use crate::schema::{find_drawing_layer, layer_base};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetLayerVisible, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_visible(payload.layer_id.clone(), layer_base(layer).visible)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
