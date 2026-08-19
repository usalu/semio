//! ↩️ `delete-edge` — reconstructs the removed edge from BASE (captured verbatim). Missing target
//! ⇒ `Vec::new()`.

use super::mutation::DeleteEdge;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{create_edge, SemioGraphMutation};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteEdge, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    match base.edges.iter().find(|e| e.id == payload.id) {
        Some(edge) => vec![SemioGraphMutation::CreateEdge(create_edge::mutation::CreateEdge { id: edge.id.clone(), source: edge.source.clone(), target: edge.target.clone(), kind: edge.kind.clone(), label: edge.label.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
