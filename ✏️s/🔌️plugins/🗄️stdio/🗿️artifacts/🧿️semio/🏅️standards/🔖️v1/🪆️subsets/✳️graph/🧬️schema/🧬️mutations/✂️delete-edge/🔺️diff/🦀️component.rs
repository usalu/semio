//! 🔺️ `delete-edge` — sparse diff construction; Error `mutation.target-missing` when the addressed
//! edge is absent (no cascade needed — edges don't own other entities).

use super::mutation::DeleteEdge;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteEdge, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if !base.edges.iter().any(|e| e.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    }
    let mut edges = base.edges.clone();
    edges.retain(|e| e.id != payload.id);
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: edges }) })
}
//#endregion 🔖️Diff
