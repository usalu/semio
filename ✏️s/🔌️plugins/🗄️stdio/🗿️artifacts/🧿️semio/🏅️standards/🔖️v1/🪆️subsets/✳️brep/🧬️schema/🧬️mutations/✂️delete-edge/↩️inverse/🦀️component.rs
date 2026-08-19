//! ↩️ `delete-edge` — reconstructs the removed edge from BASE via `CreateEdge`.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteEdge;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_edge, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteEdge, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    match base.edges.iter().find(|x| x.id == payload.id) {
        Some(x) => vec![SemioBrepMutation::CreateEdge(create_edge::mutation::CreateEdge { id: x.id.clone(), start_vertex: x.start_vertex.clone(), end_vertex: x.end_vertex.clone(), curve: x.curve.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
