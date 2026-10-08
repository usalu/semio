//! ↩️ Inverse for `SetNodeLabel`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetNodeLabel, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::SetNodeLabel { id, .. } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id: id.clone(), label: node.label.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
