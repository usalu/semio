//! 🔺️ Sparse diff builder for `DeleteNode` — a real cascade-aware removal (node + any edge that
//! touches it), never a whole-snapshot capture.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::diff::text::diff_replace_content;
use crate::artifacts::dag::schema::split_endpoint;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> protocol::MutationOutcome<DagDiff> {
    let scene = dag_working_scene(base);
    if !scene.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let cascaded_edge_ids: Vec<String> = scene.edges.iter().filter(|edge| split_endpoint(&edge.source).0 == payload.id || split_endpoint(&edge.target).0 == payload.id).map(|edge| edge.id.clone()).collect();
    let nodes: Vec<_> = scene.nodes.into_iter().filter(|node| node.id != payload.id).collect();
    let edges: Vec<_> = scene.edges.into_iter().filter(|edge| split_endpoint(&edge.source).0 != payload.id && split_endpoint(&edge.target).0 != payload.id).collect();
    let outcome = protocol::MutationOutcome::new(diff_replace_content(nodes, edges));
    if cascaded_edge_ids.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting node \"{}\" also removed {} connected edge(s): {}.", payload.id, cascaded_edge_ids.len(), cascaded_edge_ids.join(", ")))
    }
}
//#endregion 🔖️Diff
