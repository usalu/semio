//! ↩️ `delete-vertex` — a real multi-mutation cascade: one `CreateVertex` followed by one
//! `CreateEdge` per severed edge (in `base.edges` order), reconstructed entirely from BASE.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteVertex;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_edge, create_vertex, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteVertex, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(vertex) = base.vertices.iter().find(|v| v.id == payload.id) else {
        return Vec::new();
    };
    let mut out = vec![SemioBrepMutation::CreateVertex(create_vertex::mutation::CreateVertex { id: vertex.id.clone(), point: vertex.point })];
    for edge in base.edges.iter().filter(|e| e.start_vertex == payload.id || e.end_vertex == payload.id) {
        out.push(SemioBrepMutation::CreateEdge(create_edge::mutation::CreateEdge { id: edge.id.clone(), start_vertex: edge.start_vertex.clone(), end_vertex: edge.end_vertex.clone(), curve: edge.curve.clone() }));
    }
    out
}
//#endregion 🔖️Inverse
