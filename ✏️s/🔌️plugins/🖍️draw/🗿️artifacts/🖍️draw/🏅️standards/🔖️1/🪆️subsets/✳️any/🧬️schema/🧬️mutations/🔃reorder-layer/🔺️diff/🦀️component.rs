//! 🔺️ Sparse diff builder for `ReorderLayer` — a real handcrafted remove+insert at the new
//! address, never apply-then-capture.
use crate::artifacts::draw::diff::{diff_reorder_layer, DrawDiff};
use crate::artifacts::draw::schema::{find_draw_layer, find_draw_layer_location};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReorderLayer, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if let Some(parent_id) = payload.parent_id.as_deref() {
        if find_draw_layer(base, parent_id).is_none() {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Parent layer \"{}\" does not exist.", parent_id), [parent_id.to_string()]);
        }
    }
    if let Some(location) = find_draw_layer_location(base, &payload.layer_id) {
        if location.parent_id.as_deref() == payload.parent_id.as_deref() && location.index == payload.index {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" is already at that position.", payload.layer_id));
        }
    }
    protocol::MutationOutcome::new(diff_reorder_layer(&payload.layer_id, payload.parent_id.as_deref(), payload.index, layer.clone()))
}
//#endregion 🔖️Diff
