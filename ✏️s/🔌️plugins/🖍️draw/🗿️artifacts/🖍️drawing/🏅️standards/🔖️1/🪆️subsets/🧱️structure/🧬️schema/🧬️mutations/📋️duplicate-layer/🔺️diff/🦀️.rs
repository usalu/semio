//! 🔺️ Sparse diff builder for `DuplicateLayer` — real handcrafted insert of the cloned subtree
//! right after its source, never apply-then-capture.
use crate::artifacts::drawing::diff::{diff_create_layer, DrawingDiff};
use crate::artifacts::drawing::schema::{clone_drawing_layer_node, find_drawing_layer, find_drawing_layer_location};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DuplicateLayer, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    let duplicate = clone_drawing_layer_node(layer, " copy");
    let new_id = crate::artifacts::drawing::schema::layer_id(&duplicate);
    if find_drawing_layer(base, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A layer with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    let diff = match find_drawing_layer_location(base, &payload.layer_id) {
        Some(location) => diff_create_layer(location.parent_id.as_deref(), location.index + 1, duplicate),
        None => diff_create_layer(None, base.layers.len(), duplicate),
    };
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
