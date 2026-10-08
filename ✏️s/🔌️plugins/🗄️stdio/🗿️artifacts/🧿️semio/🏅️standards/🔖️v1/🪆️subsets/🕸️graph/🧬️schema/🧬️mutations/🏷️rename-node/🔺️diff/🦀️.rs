//! 🔺️ Diff for `RenameNode`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

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
    let at = base.nodes.iter().position(|node| node.id == payload.id).expect("checked above");
    let edges: Vec<IndexModified<SemioGraphEdgeDiff>> = base
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| edge.source == payload.id || edge.target == payload.id)
        .map(|(index, edge)| IndexModified { index, diff: SemioGraphEdgeDiff { source: (edge.source == payload.id).then(|| payload.new_id.clone()), target: (edge.target == payload.id).then(|| payload.new_id.clone()), ..Default::default() } })
        .collect();
    protocol::MutationOutcome::new(SemioGraphDiff {
        nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { id: Some(payload.new_id.clone()), ..Default::default() } }], ..Default::default() }),
        edges: (!edges.is_empty()).then(|| IndexedTripleDiff { modified: edges, ..Default::default() }),
    })
}
//#endregion 🔖️Diff
