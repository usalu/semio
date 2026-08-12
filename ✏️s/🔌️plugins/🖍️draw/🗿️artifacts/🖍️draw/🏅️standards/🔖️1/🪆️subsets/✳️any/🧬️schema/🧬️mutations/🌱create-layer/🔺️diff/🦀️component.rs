//! 🔺️ Sparse diff builder for `CreateLayer` — a real parent-aware insert (never a whole-snapshot
//! capture), resolving a `None` index against BASE's target-list length.
use crate::artifacts::draw::diff::{diff_create_layer, DrawDiff};
use crate::artifacts::draw::engine::find_draw_layer;
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

pub fn diff(payload: &super::mutation::CreateLayer, base: &DrawSnapshot) -> DrawDiff {
    let index = payload.index.unwrap_or_else(|| append_index(base, payload.parent_id.as_deref()));
    diff_create_layer(payload.parent_id.as_deref(), index, (*payload.layer).clone())
}
//#endregion 🔖️Diff
