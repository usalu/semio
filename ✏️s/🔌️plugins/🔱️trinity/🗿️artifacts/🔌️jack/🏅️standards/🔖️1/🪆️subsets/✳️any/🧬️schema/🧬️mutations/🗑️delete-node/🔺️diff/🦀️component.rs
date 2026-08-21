//! 🔺️ Sparse diff builder for `DeleteNode` — removes the node AND every edge severed by its
//! removal (real cascade capture, never apply-then-capture) against the current scene off `base`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteNode, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let scene = crate::artifacts::jack::jack_working_scene(base);
    if !scene.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let cascaded_edge_ids: Vec<String> =
        scene.edges.iter().filter(|edge| crate::artifacts::jack::port_node_id(&edge.source) == Some(payload.id.as_str()) || crate::artifacts::jack::port_node_id(&edge.target) == Some(payload.id.as_str())).map(|edge| edge.id.clone()).collect();
    let nodes: Vec<_> = scene.nodes.into_iter().filter(|node| node.id != payload.id).collect();
    let edges: Vec<_> = scene.edges.into_iter().filter(|edge| crate::artifacts::jack::port_node_id(&edge.source) != Some(payload.id.as_str()) && crate::artifacts::jack::port_node_id(&edge.target) != Some(payload.id.as_str())).collect();
    let outcome = protocol::MutationOutcome::new(diff_replace_content(nodes, edges));
    if cascaded_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting node \"{}\" also removed {} connected edge(s): {}.", payload.id, cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")))
    }
}
//#endregion 🔖️Diff
