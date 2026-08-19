//! 🔺️ Sparse diff builder for `DuplicateLayer` — real handcrafted insert of the cloned subtree
//! right after its source, never apply-then-capture.
use crate::artifacts::draw::diff::{diff_create_layer, DrawDiff};
use crate::artifacts::draw::schema::{clone_draw_layer_node, find_draw_layer, find_draw_layer_location};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DuplicateLayer, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    let duplicate = clone_draw_layer_node(layer, " copy");
    let new_id = crate::artifacts::draw::schema::layer_id(&duplicate);
    if find_draw_layer(base, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A layer with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    let diff = match find_draw_layer_location(base, &payload.layer_id) {
        Some(location) => diff_create_layer(location.parent_id.as_deref(), location.index + 1, duplicate),
        None => diff_create_layer(None, base.layers.len(), duplicate),
    };
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
