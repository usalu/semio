//! ↩️ Inverse for `DeleteEdge`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, create_edge, delete_edge};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteEdge, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.edges.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.edges[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteEdge(delete_edge::DeleteEdge { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| {
        SemioBrepMutation::CreateEdge(create_edge::CreateEdge { id: x.id.clone(), start_vertex: x.start_vertex.clone(), end_vertex: x.end_vertex.clone(), curve: x.curve.clone() })
    }));
    undo
}
//#endregion 🔖️Inverse
