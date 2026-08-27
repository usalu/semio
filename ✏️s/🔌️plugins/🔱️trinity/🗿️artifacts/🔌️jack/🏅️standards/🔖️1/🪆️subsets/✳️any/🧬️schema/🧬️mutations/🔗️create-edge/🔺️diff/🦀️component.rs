//! 🔺️ Sparse diff builder for `CreateEdge` — a real append-only insert.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateEdge, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let scene = crate::artifacts::jack::jack_working_scene(base);
    if scene.edges.iter().any(|edge| edge.id == payload.edge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.edge.id), [payload.edge.id.clone()]);
    }
    let source_id = crate::artifacts::jack::port_node_id(&payload.edge.source);
    let target_id = crate::artifacts::jack::port_node_id(&payload.edge.target);
    let endpoints_exist = source_id.is_some_and(|id| scene.nodes.iter().any(|node| node.id == id)) && target_id.is_some_and(|id| scene.nodes.iter().any(|node| node.id == id));
    if !endpoints_exist {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" references an endpoint that does not exist ({} -> {}).", payload.edge.id, payload.edge.source, payload.edge.target), [payload.edge.id.clone()]);
    }
    let mut edges = scene.edges;
    edges.push(payload.edge.clone());
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, edges))
}
//#endregion 🔖️Diff
