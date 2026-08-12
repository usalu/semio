//! 🔺️ Sparse diff builder for `DuplicateLayer` — real handcrafted insert of the cloned subtree
//! right after its source, never apply-then-capture.
use crate::artifacts::draw::diff::{diff_create_layer, DrawDiff};
use crate::artifacts::draw::engine::{clone_draw_layer_node, find_draw_layer, find_draw_layer_location};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DuplicateLayer, base: &DrawSnapshot) -> DrawDiff {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return DrawDiff::default();
    };
    let duplicate = clone_draw_layer_node(layer, " copy");
    match find_draw_layer_location(base, &payload.layer_id) {
        Some(location) => diff_create_layer(location.parent_id.as_deref(), location.index + 1, duplicate),
        None => diff_create_layer(None, base.layers.len(), duplicate),
    }
}
//#endregion 🔖️Diff
