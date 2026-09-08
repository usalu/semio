//! ↩️ Inverse for `DeleteLayer` — reconstructs a `create-layer` at the exact captured (parent,
//! index) BASE location, carrying the full removed subtree (children included for a group).
//! Missing target ⇒ `Vec::new()`.
use crate::mutations::DrawingMutation;
use crate::schema::{find_drawing_layer, find_drawing_layer_location};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteLayer, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    let (Some(layer), Some(location)) = (find_drawing_layer(base, &payload.layer_id), find_drawing_layer_location(base, &payload.layer_id)) else {
        return Vec::new();
    };
    vec![crate::mutations::create_layer(location.parent_id, Some(location.index), layer.clone())]
}
//#endregion 🔖️Inverse
