//! ↩️ Inverse for `ReorderLayer` — the OLD `(parent_id, index)` address captured from BASE. Missing
//! target ⇒ `Vec::new()`.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::schema::find_draw_layer_location;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReorderLayer, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer_location(base, &payload.layer_id) {
        Some(location) => vec![super::mutation::reorder_layer(payload.layer_id.clone(), location.parent_id, location.index)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
