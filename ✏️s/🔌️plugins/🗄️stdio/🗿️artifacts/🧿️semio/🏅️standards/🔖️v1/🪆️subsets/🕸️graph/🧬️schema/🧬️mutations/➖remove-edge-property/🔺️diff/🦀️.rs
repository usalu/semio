//! 🔺️ Diff for `RemoveEdgeProperty`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
/// 🧮️ An edge the graph lacks, or a key it does not carry, is `mutation.target-missing`; otherwise that entry leaves.
pub fn diff(payload: &super::RemoveEdgeProperty, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(position) = base.edges.iter().position(|edge| edge.id == payload.edge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.edge_id.value), [payload.edge_id.value.clone()]);
    };
    let Some(index) = base.edges[position].properties.iter().position(|property| property.key == payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" has no property \"{}\".", payload.edge_id.value, payload.key), [payload.edge_id.value.clone(), payload.key.clone()]);
    };
    let properties = IndexedTripleDiff { removed: vec![index], ..Default::default() };
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: None, edges: Some(IndexedTripleDiff { modified: vec![IndexModified { index: position, diff: SemioGraphEdgeDiff { properties: Some(properties), ..Default::default() } }], ..Default::default() }) })
}
//#endregion 🔖️Diff
