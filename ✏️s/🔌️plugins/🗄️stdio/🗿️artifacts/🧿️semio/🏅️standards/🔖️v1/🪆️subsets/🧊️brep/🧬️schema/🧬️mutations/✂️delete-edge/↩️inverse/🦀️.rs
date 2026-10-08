//! ↩️ Inverse for `DeleteEdge`.

use crate::standards::v1::subsets::brep::schema::mutations::{create_edge, delete_edge, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteEdge, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(index) = base.edges.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let x = &base.edges[index];
    vec![SemioBrepMutation::CreateEdge(create_edge::CreateEdge { id: x.id.clone(), start_vertex: x.start_vertex.clone(), end_vertex: x.end_vertex.clone(), curve: x.curve.clone(), tol: x.tol, at: Some(index) })]

    })())
}
//#endregion 🔖️Inverse
