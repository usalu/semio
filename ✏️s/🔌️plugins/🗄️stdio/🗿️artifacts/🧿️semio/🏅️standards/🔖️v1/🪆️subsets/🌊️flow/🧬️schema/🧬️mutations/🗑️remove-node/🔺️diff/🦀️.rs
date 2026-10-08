//! 🔺️ Diff for `RemoveNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveNode, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::RemoveNode { id } = payload;
    protocol::MutationOutcome::new(diff_remove_node(id))
}
//#endregion 🔖️Diff
