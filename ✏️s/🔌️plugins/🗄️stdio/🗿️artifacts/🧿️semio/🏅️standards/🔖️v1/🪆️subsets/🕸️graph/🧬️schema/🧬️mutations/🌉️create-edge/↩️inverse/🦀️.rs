//! ↩️ Inverse for `CreateEdge`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, delete_edge};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, GraphNodeId, SemioGraphEdge, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateEdge, _base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    vec![SemioGraphMutation::DeleteEdge(delete_edge::DeleteEdge { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
