//! ↩️ Inverse for `SetNodeParam`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetNodeParam, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::SetNodeParam { id, key, .. } = payload;
    Ok(match param_value_at(base, id, key) {
        Some(value) => vec![SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: id.clone(), key: key.clone(), value: value.to_string(), at: param_index_at(base, id, key) })],
        None => vec![SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id: id.clone(), key: key.clone() })],
    })
}
//#endregion 🔖️Inverse
