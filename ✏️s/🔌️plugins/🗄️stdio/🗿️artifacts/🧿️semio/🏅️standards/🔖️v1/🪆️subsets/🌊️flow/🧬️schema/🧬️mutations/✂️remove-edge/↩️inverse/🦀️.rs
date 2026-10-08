//! ↩️ Inverse for `RemoveEdge`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveEdge, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::RemoveEdge { id } = payload;
    Ok(match edge_at(base, id) {
        Some(edge) => vec![SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge: edge.clone(), at: base.edges.iter().position(|e| e.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
