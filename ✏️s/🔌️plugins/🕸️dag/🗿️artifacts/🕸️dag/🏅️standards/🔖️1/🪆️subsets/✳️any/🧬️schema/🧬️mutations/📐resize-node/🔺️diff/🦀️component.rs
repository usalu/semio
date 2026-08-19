//! 🔺️ Sparse diff builder for `ResizeNode`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ResizeNode, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    let Some(existing) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !payload.width.is_finite() || !payload.height.is_finite() || payload.width <= 0.0 || payload.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Node \"{}\" size must be finite and positive, got ({}, {}).", payload.id, payload.width, payload.height), [payload.id.clone()]);
    }
    if existing.width == payload.width && existing.height == payload.height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" already has size ({}, {}).", payload.id, payload.width, payload.height));
    }
    let mut nodes = scene.nodes;
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.width = payload.width;
        node.height = payload.height;
    }
    protocol::MutationOutcome::new(diff_replace_content(nodes, scene.edges))
}
//#endregion 🔖️Diff
