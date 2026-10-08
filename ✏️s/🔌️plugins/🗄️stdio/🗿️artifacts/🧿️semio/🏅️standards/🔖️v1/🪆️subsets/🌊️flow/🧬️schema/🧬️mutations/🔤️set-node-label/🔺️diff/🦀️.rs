//! 🔺️ Diff for `SetNodeLabel`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetNodeLabel, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::SetNodeLabel { id, label } = payload;
    protocol::MutationOutcome::new(diff_set_node_label(id, label))
}
//#endregion 🔖️Diff
