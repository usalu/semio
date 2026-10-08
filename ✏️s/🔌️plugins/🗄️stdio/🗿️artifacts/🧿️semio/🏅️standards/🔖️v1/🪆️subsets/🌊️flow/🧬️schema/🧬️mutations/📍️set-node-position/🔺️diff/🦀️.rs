//! 🔺️ Diff for `SetNodePosition`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetNodePosition, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::SetNodePosition { id, position } = payload;
    protocol::MutationOutcome::new(diff_set_node_position(id, *position))
}
//#endregion 🔖️Diff
