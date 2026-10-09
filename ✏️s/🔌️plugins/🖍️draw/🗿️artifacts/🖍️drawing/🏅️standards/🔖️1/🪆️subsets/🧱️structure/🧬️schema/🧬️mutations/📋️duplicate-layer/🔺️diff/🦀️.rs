//! 🔺️ Sparse diff builder for `DuplicateLayer` — real handcrafted insert of the cloned subtree
//! right after its source, never apply-then-capture.
use crate::diff::{diff_create_layer, DrawingDiff};
use crate::schema::{clone_drawing_layer_node, find_drawing_layer, find_drawing_layer_location};
use crate::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DuplicateLayer, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]);
    };
    let duplicate = match clone_drawing_layer_node(layer, " copy", &payload.identities) {Ok(layer)=>layer,Err(error)=>return protocol::MutationOutcome::error("mutation.identity-invalid",error.to_string(),[payload.layer_id.to_string_owner()])};
    for assignment in &payload.identities {if find_drawing_layer(base,&assignment.target).is_some(){return protocol::MutationOutcome::fatal("mutation.duplicate-id","A clone identity already exists.",[assignment.target.to_string_owner()]);}}
    let new_id = crate::schema::layer_id(&duplicate);
    if find_drawing_layer(base, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A layer with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    let diff = match find_drawing_layer_location(base, &payload.layer_id) {
        Some(location) => diff_create_layer(&base.layers, location.parent_id.as_ref(), location.index + 1, duplicate),
        None => diff_create_layer(&base.layers, None, base.layers.len(), duplicate),
    };
    protocol::MutationOutcome::new(diff)
}
//#endregion 🔖️Diff
