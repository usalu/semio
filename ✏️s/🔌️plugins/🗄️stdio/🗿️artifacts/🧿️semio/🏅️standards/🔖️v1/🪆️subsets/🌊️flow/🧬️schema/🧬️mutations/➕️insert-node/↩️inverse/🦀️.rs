//! ↩️ Inverse for `InsertNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertNode, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::InsertNode { node, .. } = payload;
    Ok(vec![SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: node.id.clone() })])
}
//#endregion 🔖️Inverse
