//! 🔺️ Sparse diff builder for `MoveNode`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MoveNode, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    let Some(existing) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" position must be finite, got ({}, {}).", payload.id, payload.x, payload.y), [payload.id.clone()]);
    }
    if existing.x == payload.x && existing.y == payload.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.id, payload.x, payload.y));
    }
    if let Some(node) = scene.nodes.iter_mut().find(|node| node.id == payload.id) {
        node.x = payload.x;
        node.y = payload.y;
    }
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, scene.edges))
}
//#endregion 🔖️Diff
