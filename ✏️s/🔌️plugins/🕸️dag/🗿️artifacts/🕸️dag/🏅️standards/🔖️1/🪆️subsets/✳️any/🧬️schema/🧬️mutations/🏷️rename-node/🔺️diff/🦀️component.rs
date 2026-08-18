//! 🔺️ Sparse diff builder for `RenameNode` — patches the node's `id` in place (no reorder side
//! effect) and rewrites every edge endpoint string that pointed at the old id.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::schema::split_endpoint;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};
use infinite_board_port_directed_dag::DagEdgePatch;
use protocol::Patchable;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenameNode, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    if !scene.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if payload.new_id == payload.id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" already has that id.", payload.id));
    }
    if scene.nodes.iter().any(|node| node.id == payload.new_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.new_id), [payload.new_id.clone()]);
    }
    let mut nodes = scene.nodes;
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.id = payload.new_id.clone();
    }
    let edges: Vec<_> = scene
        .edges
        .into_iter()
        .map(|mut edge| {
            let (source_node, source_port) = split_endpoint(&edge.source);
            let (target_node, target_port) = split_endpoint(&edge.target);
            let touches_source = source_node == payload.id;
            let touches_target = target_node == payload.id;
            if touches_source || touches_target {
                let patch = DagEdgePatch { source: touches_source.then(|| format!("{}@{}", payload.new_id, source_port)), target: touches_target.then(|| format!("{}@{}", payload.new_id, target_port)) };
                edge.apply_patch(&patch);
            }
            edge
        })
        .collect();
    protocol::MutationOutcome::new(diff_replace_content(nodes, edges))
}
//#endregion 🔖️Diff
