//! ↩️ Inverse for `DeleteEdge`.

use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{SemioGraphMutation, create_edge};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphEdgeId, SemioGraphSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteEdge, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.edges.iter().find(|e| e.id == payload.id) {
        Some(edge) => vec![SemioGraphMutation::CreateEdge(create_edge::CreateEdge { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), kind: edge.kind.clone(), label: edge.label.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
