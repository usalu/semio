//! 🔺️ Sparse diff builder for `ReorderLayer` — a real handcrafted remove+insert at the new
//! address, never apply-then-capture.
use crate::artifacts::draw::diff::{diff_reorder_layer, DrawDiff};
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReorderLayer, base: &DrawSnapshot) -> DrawDiff {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => diff_reorder_layer(&payload.layer_id, payload.parent_id.as_deref(), payload.index, layer.clone()),
        None => DrawDiff::default(),
    }
}
//#endregion 🔖️Diff
