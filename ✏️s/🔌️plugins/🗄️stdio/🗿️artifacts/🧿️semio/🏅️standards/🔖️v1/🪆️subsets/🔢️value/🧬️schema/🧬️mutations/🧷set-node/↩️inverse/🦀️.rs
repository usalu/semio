//! ↩️ Inverse for `SetNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetNode, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::SetNode { id, .. } = payload;
    Ok(match base.nodes.iter().find(|n| &n.id == id) {
        Some(existing) => vec![SemioValueMutation::SetNode(set_node::SetNode { id: id.clone(), value: existing.value.clone() })],
        None => vec![SemioValueMutation::RemoveNode(remove_node::RemoveNode { id: id.clone() })],
    })
}
//#endregion 🔖️Inverse
