//! ↩️ Inverse for `SetNodeKind`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetNodeKind, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    let super::SetNodeKind { id, .. } = payload;
    Ok(match node_at(base, id) {
        Some(node) => vec![SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id: id.clone(), kind: node.kind.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
