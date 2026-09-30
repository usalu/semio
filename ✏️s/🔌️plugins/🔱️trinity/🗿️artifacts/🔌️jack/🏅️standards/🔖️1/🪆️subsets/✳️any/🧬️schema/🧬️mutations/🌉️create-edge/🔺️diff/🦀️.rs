//! 🔺️ Sparse diff builder for `CreateEdge` — a real append-only insert.
use crate::standards::v1::subsets::any::schema::diff::{diff_replace_content, JackDiff};
use crate::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateEdge, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let scene = crate::jack_working_scene(base);
    if scene.edges.iter().any(|edge| edge.id == payload.edge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.edge.id), [payload.edge.id.clone()]);
    }
    let (Some(source_id), Some(target_id)) = (crate::port_node_id(&payload.edge.source), crate::port_node_id(&payload.edge.target)) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" endpoints must be `nodeId@portId` port keys ({} -> {}).", payload.edge.id, payload.edge.source, payload.edge.target), [payload.edge.id.clone()]);
    };
    let mut missing: Vec<String> = [source_id, target_id].into_iter().filter(|id| !scene.nodes.iter().any(|node| node.id == *id)).map(str::to_string).collect();
    missing.dedup();
    if !missing.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" references a node that does not exist ({} -> {}).", payload.edge.id, payload.edge.source, payload.edge.target), missing);
    }
    let mut edges = scene.edges;
    edges.push(payload.edge.clone());
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, edges))
}
//#endregion 🔖️Diff
