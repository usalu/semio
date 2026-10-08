//! ↩️ Inverse for `RemoveNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveNode, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::RemoveNode { id } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![SemioFlowMutation::InsertNode(insert_node::InsertNode { node: node.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
