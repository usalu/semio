//! 🔺️ Sparse diff builder for `MoveNode`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MoveNode, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    let Some(existing) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" position must be finite, got ({}, {}).", payload.id, payload.x, payload.y), [payload.id.clone()]);
    }
    if existing.x == payload.x && existing.y == payload.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" is already at ({}, {}).", payload.id, payload.x, payload.y));
    }
    let mut nodes = scene.nodes;
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.x = payload.x;
        node.y = payload.y;
    }
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges))
}
//#endregion 🔖️Diff
