//! 🔺️ Diff for `RemoveNodeParam`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveNodeParam, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::RemoveNodeParam { id, key } = payload;
    protocol::MutationOutcome::new(diff_remove_node_param(id, key))
}
//#endregion 🔖️Diff
