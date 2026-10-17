//! ↩️ Inverse for `InsertEdge`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioFlowSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_edge;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertEdge, _base: &SemioFlowSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_edge::RemoveEdge>,
{
    let super::InsertEdge { edge, .. } = payload;
    Ok(vec![Op::from(remove_edge::RemoveEdge { id: edge.id.clone() })])
}
//#endregion 🔖️Inverse
