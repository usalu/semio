//! ↩️ Inverse for `SetNodePosition`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetNodePosition, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::SetNodePosition { id, .. } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: id.clone(), position: node.position })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
