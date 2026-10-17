//! ↩️ Inverse for `DeleteEdge`.

use crate::schema::mutations::create_edge;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DeleteEdge, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_edge::CreateEdge>,
{
    Ok((|| {
    let Some(index) = base.edges.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let x = &base.edges[index];
    vec![Op::from(create_edge::CreateEdge { id: x.id.clone(), start_vertex: x.start_vertex.clone(), end_vertex: x.end_vertex.clone(), curve: x.curve.clone(), tol: x.tol, at: Some(index) })]

    })())
}
//#endregion 🔖️Inverse
