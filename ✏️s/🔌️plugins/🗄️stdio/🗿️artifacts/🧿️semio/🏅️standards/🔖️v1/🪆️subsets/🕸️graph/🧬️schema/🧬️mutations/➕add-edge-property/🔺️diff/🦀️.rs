//! 🔺️ Diff for `AddEdgeProperty`.

use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
/// 🧮️ An empty key is a Fatal `mutation.invariant`; an edge the graph lacks is `mutation.target-missing`; a key the edge
/// already carries is `mutation.no-op` (keys are unique per edge); otherwise the entry lands at `index` (clamped to the end).
pub fn diff(payload: &super::AddEdgeProperty, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let target = [payload.edge_id.value.clone()];
    if payload.property.key.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "an edge property key is never empty", target);
    }
    let Some(edge) = base.edges.iter().find(|edge| edge.id == payload.edge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.edge_id.value), target);
    };
    if edge.properties.iter().any(|property| property.key == payload.property.key) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Edge \"{}\" already has a property \"{}\".", payload.edge_id.value, payload.property.key));
    }
    let mut edges = base.edges.clone();
    if let Some(edge) = edges.iter_mut().find(|edge| edge.id == payload.edge_id) {
        edge.properties.insert(payload.index.min(edge.properties.len()), payload.property.clone());
    }
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: edges }) })
}
//#endregion 🔖️Diff
