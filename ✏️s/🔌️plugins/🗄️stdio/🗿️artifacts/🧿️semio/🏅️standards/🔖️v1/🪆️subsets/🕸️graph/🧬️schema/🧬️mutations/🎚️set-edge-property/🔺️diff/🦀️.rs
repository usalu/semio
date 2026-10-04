//! 🔺️ Diff for `SetEdgeProperty`.

use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
/// 🧮️ An empty key is a Fatal `mutation.invariant`; an edge the graph lacks, or a key the edge lacks, is
/// `mutation.target-missing`; a value equal to the current one is `mutation.no-op`; otherwise the property takes the value
/// in place (its position among the edge's properties is kept).
pub fn diff(payload: &super::SetEdgeProperty, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let target = [payload.edge_id.value.clone()];
    if payload.key.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "an edge property key is never empty", target);
    }
    let Some(edge) = base.edges.iter().find(|edge| edge.id == payload.edge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.edge_id.value), target);
    };
    let Some(current) = edge.properties.iter().find(|property| property.key == payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" has no property \"{}\".", payload.edge_id.value, payload.key), target);
    };
    if current.value == payload.value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Property \"{}\" of edge \"{}\" already has that value.", payload.key, payload.edge_id.value));
    }
    let mut edges = base.edges.clone();
    if let Some(property) = edges.iter_mut().filter(|edge| edge.id == payload.edge_id).flat_map(|edge| edge.properties.iter_mut()).find(|property| property.key == payload.key) {
        property.value = payload.value.clone();
    }
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: edges }) })
}
//#endregion 🔖️Diff
