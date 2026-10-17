//! ↩️ Inverse for `RemoveEdge`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::edge_at;
#[allow(unused_imports)]
use crate::schema::mutations::insert_edge;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveEdge, base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_edge::InsertEdge>,
{
    let super::RemoveEdge { id } = payload;
    Ok(match edge_at(base, id) {
        Some(edge) => vec![Op::from(insert_edge::InsertEdge { edge: edge.clone(), at: base.edges.iter().position(|e| e.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
