//! ↩️ Inverse for `ReorderLayer` — the OLD `(parent_id, index)` address captured from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::find_drawing_layer_location;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReorderLayer, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer_location(base, &payload.layer_id) {
        Some(location) => vec![super::mutation::reorder_layer(payload.layer_id.clone(), location.parent_id, location.index)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
