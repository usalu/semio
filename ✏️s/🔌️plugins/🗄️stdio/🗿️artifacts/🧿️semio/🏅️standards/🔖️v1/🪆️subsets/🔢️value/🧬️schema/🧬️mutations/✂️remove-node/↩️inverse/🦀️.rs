//! ↩️ Inverse for `RemoveNode`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveNode, base: &SemioValueSnapshot) -> Result<Vec<SemioValueMutation>, semio_framework_value::ValueError> {
    let super::RemoveNode { id } = payload;
    Ok(match base.nodes.iter().find(|n| &n.id == id) {
        Some(existing) => vec![SemioValueMutation::SetNode(set_node::SetNode { id: id.clone(), value: existing.value.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
