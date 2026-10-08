//! ↩️ Inverse for `SetEdgeEndpoints`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetEdgeEndpoints, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::SetEdgeEndpoints { id, .. } = payload;
    Ok(match edge_at(base, id) {
        Some(edge) => vec![SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: id.clone(), from: edge.from.clone(), to: edge.to.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
