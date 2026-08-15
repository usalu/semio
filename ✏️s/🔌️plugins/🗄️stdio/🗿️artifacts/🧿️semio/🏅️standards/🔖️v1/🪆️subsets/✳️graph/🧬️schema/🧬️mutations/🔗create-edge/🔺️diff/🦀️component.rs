//! 🔺️ `create-edge` — sparse diff construction; if an edge with this `id` already exists in
//! `base`, this is a no-op (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateEdge;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEdgeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateEdge, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut edges = base.edges.clone();
    if !edges.iter().any(|e| e.id == payload.id) {
        edges.push(SemioGraphEdge { id: payload.id.clone(), source: payload.source.clone(), target: payload.target.clone(), kind: payload.kind.clone(), label: payload.label.clone() });
    }
    SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: edges }) }
}
//#endregion 🔖️Diff
