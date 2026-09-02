//! ↩️ Inverse for `DeleteLayer` — reconstructs a `create-layer` at the exact captured (parent,
//! index) BASE location, carrying the full removed subtree (children included for a group).
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::{find_drawing_layer, find_drawing_layer_location};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteLayer, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    let (Some(layer), Some(location)) = (find_drawing_layer(base, &payload.layer_id), find_drawing_layer_location(base, &payload.layer_id)) else {
        return Vec::new();
    };
    vec![crate::artifacts::drawing::mutations::create_layer::mutation::create_layer(location.parent_id, Some(location.index), layer.clone())]
}
//#endregion 🔖️Inverse
