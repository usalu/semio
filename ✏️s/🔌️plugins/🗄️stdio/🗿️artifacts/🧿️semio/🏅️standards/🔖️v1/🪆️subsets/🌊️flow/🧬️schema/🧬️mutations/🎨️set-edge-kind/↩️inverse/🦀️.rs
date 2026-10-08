//! ↩️ Inverse for `SetEdgeKind`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetEdgeKind, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::SetEdgeKind { id, .. } = payload;
    Ok(match edge_at(base, id) {
        Some(edge) => vec![SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id: id.clone(), kind: edge.kind.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
