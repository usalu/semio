//! 🔺️ Sparse diff builder for `CreateLayer` — a real parent-aware insert (never a whole-snapshot
//! capture), resolving a `None` index against BASE's target-list length.
use crate::artifacts::draw::diff::{diff_create_layer, DrawDiff};
use crate::artifacts::draw::schema::find_draw_layer;
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};

//#region 🔖️Diff
/// 📐️ Resolves the FINAL-state append index for `parent_id` (root length, or the target group's
/// current child count) when the payload didn't pin one.
fn append_index(base: &DrawSnapshot, parent_id: Option<&str>) -> usize {
    match parent_id.and_then(|id| find_draw_layer(base, id)) {
        Some(DrawLayerNode::Group(group)) => group.children.len(),
        _ => base.layers.len(),
    }
}

pub fn diff(payload: &super::mutation::CreateLayer, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let new_id = crate::artifacts::draw::schema::layer_id(&payload.layer);
    if find_draw_layer(base, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A layer with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    if let Some(parent_id) = payload.parent_id.as_deref() {
        match find_draw_layer(base, parent_id) {
            Some(DrawLayerNode::Group(_)) => {}
            _ => return protocol::MutationOutcome::fatal("mutation.invariant", format!("Parent layer \"{}\" does not exist or is not a group.", parent_id), [parent_id.to_string()]),
        }
    }
    let index = payload.index.unwrap_or_else(|| append_index(base, payload.parent_id.as_deref()));
    protocol::MutationOutcome::new(diff_create_layer(payload.parent_id.as_deref(), index, (*payload.layer).clone()))
}
//#endregion 🔖️Diff
