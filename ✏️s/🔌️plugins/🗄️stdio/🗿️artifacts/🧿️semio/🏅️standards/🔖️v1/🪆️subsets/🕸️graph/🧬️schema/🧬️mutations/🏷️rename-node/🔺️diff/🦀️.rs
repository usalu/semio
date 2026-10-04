//! 🔺️ Diff for `RenameNode`.

use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList, SemioGraphNodeList};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
/// 🧮️ An empty new id is a Fatal `mutation.invariant` (the schema's hard bound); a node the graph lacks is
/// `mutation.target-missing`; the node's own id is `mutation.no-op`; an id another node carries is a Fatal
/// `mutation.duplicate-id`. Otherwise the node takes the id in place and every edge endpoint naming it follows.
pub fn diff(payload: &super::RenameNode, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let target = [payload.id.value.clone()];
    if payload.new_id.value.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node id is never empty", target);
    }
    if !base.nodes.iter().any(|node| node.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id.value), target);
    }
    if payload.new_id == payload.id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" already has that id.", payload.id.value));
    }
    if base.nodes.iter().any(|node| node.id == payload.new_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A node with id \"{}\" already exists.", payload.new_id.value), [payload.new_id.value.clone()]);
    }
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.id = payload.new_id.clone();
    }
    let mut edges = base.edges.clone();
    let mut touched = false;
    for edge in &mut edges {
        for endpoint in [&mut edge.source, &mut edge.target] {
            if *endpoint == payload.id {
                *endpoint = payload.new_id.clone();
                touched = true;
            }
        }
    }
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: touched.then_some(SemioGraphEdgeList { values: edges }) })
}
//#endregion 🔖️Diff
