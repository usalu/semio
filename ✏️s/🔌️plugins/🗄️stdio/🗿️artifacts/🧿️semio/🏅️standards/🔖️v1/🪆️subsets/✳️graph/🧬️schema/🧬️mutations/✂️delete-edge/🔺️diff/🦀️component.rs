//! 🔺️ `delete-edge` — sparse diff construction; an absent `id` is a no-op clone (no cascade needed
//! — edges don't own other entities).

use super::mutation::DeleteEdge;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteEdge, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut edges = base.edges.clone();
    edges.retain(|e| e.id != payload.id);
    SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: edges }) }
}
//#endregion 🔖️Diff
