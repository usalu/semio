//! ↩️ Inverse for `InsertEdge`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertEdge, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::InsertEdge { edge, .. } = payload;
    Ok(vec![SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: edge.id.clone() })])
}
//#endregion 🔖️Inverse
