//! ↩️ Inverse for `SetLayerVisible` — real logic reconstructing the undo from BASE state, never
//! from post-state. Missing target ⇒ `Vec::new()` (the semantic replacement for `NoMutation`).
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetLayerVisible, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_visible(payload.layer_id.clone(), layer_base(layer).visible)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
