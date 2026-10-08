//! ↩️ Inverse for `DeleteVertex`.

use crate::standards::v1::subsets::brep::schema::mutations::{create_edge, create_vertex, delete_edge, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteVertex, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(index) = base.vertices.iter().position(|v| v.id == payload.id) else {
        return Vec::new();
    };
    let v = &base.vertices[index];
    let mut undo = vec![SemioBrepMutation::CreateVertex(create_vertex::CreateVertex { id: v.id.clone(), point: v.point, tol: v.tol, at: Some(index) })];
    undo.extend(base.edges.iter().enumerate().filter(|(_, e)| e.start_vertex == payload.id || e.end_vertex == payload.id).map(|(at, e)| SemioBrepMutation::CreateEdge(create_edge::CreateEdge { id: e.id.clone(), start_vertex: e.start_vertex.clone(), end_vertex: e.end_vertex.clone(), curve: e.curve.clone(), tol: e.tol, at: Some(at) })));
    undo.reverse();
    undo

    })())
}
//#endregion 🔖️Inverse
