//! 🔺️ Sparse diff builder for `CreateLayer` — a real parent-aware insert (never a whole-snapshot
//! capture), resolving a `None` index against BASE's target-list length.
use semio_s_artifact_draw_drawing_diff::{diff_create_layer, DrawingDiff};
use semio_s_artifact_draw_drawing_model::find_drawing_layer;
use semio_s_artifact_draw_drawing_model::DrawingLayerNode;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Diff
/// 📐️ Resolves the FINAL-state append index for `parent_id` (root length, or the target group's
/// current child count) when the payload didn't pin one.
fn append_index(base: &DrawingSnapshot, parent_id: Option<&semio_framework_value::paged::PagedUtf8<{usize::MAX}>>) -> usize {
    match parent_id.and_then(|id| find_drawing_layer(base, id)) {
        Some(DrawingLayerNode::Group(group)) => group.children.len(),
        _ => base.layers.len(),
    }
}

pub fn diff(payload: &super::mutation::CreateLayer, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let new_id = semio_s_artifact_draw_drawing_model::layer_id(&payload.layer);
    if find_drawing_layer(base, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A layer with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    if let Some(parent_id) = payload.parent_id.as_ref() {
        match find_drawing_layer(base, parent_id) {
            Some(DrawingLayerNode::Group(_)) => {}
            _ => return protocol::MutationOutcome::fatal("mutation.invariant", format!("Parent layer \"{}\" does not exist or is not a group.", parent_id), [parent_id.to_string()]),
        }
    }
    let index = payload.index.unwrap_or_else(|| append_index(base, payload.parent_id.as_ref()));
    protocol::MutationOutcome::new(diff_create_layer(&base.layers, payload.parent_id.as_ref(), index, (*payload.layer).clone()))
}
//#endregion 🔖️Diff
