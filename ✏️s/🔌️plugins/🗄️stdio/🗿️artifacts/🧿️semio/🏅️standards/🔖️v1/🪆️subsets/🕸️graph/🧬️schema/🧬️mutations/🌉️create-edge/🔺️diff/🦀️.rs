//! 🔺️ Diff for `CreateEdge`.

use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphSnapshot};

//#region 🔖️Diff
/// 🧮️ An edge id the graph already carries is a Fatal `mutation.duplicate-id`, an unknown endpoint a Fatal `mutation.invariant`;
/// otherwise the edge is inserted at `at` (clamped to the end) or appended when `at` is absent — `delete-edge`'s and
/// `delete-node`'s exact undo name the removed edge's index.
pub fn diff(payload: &super::CreateEdge, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if base.edges.iter().any(|e| e.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id.value), [payload.id.value.clone()]);
    }
    if !base.nodes.iter().any(|n| n.id == payload.source) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" references unknown source node \"{}\".", payload.id.value, payload.source.value), [payload.id.value.clone(), payload.source.value.clone()]);
    }
    if !base.nodes.iter().any(|n| n.id == payload.target) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Edge \"{}\" references unknown target node \"{}\".", payload.id.value, payload.target.value), [payload.id.value.clone(), payload.target.value.clone()]);
    }
    let mut edges = base.edges.clone();
    edges.insert(payload.at.map_or(edges.len(), |at| at.min(edges.len())), SemioGraphEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), kind: payload.kind.clone(), label: payload.label.clone(), source_port:payload.source_port.clone(),target_port:payload.target_port.clone(),properties:payload.properties.clone() });
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: edges }) })
}
//#endregion 🔖️Diff
