//! ↩️ Inverse for `DeleteLayer` — reconstructs a `create-layer` at the exact captured (parent,
//! index) BASE location, carrying the full removed subtree (children included for a group).
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::draw::schema::{find_draw_layer, find_draw_layer_location};
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteLayer, base: &DrawSnapshot) -> Vec<DrawMutation> {
    let (Some(layer), Some(location)) = (find_draw_layer(base, &payload.layer_id), find_draw_layer_location(base, &payload.layer_id)) else {
        return Vec::new();
    };
    vec![crate::artifacts::draw::mutations::create_layer::mutation::create_layer(location.parent_id, Some(location.index), layer.clone())]
}
//#endregion 🔖️Inverse
