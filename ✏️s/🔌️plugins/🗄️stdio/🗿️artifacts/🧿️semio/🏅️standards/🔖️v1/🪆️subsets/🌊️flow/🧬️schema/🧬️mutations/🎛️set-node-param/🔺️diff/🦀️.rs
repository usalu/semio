//! 🔺️ Diff for `SetNodeParam`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetNodeParam, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::SetNodeParam { id, key, value } = payload;
    protocol::MutationOutcome::new(diff_set_node_param(base, id, key, value))
}
//#endregion 🔖️Diff
