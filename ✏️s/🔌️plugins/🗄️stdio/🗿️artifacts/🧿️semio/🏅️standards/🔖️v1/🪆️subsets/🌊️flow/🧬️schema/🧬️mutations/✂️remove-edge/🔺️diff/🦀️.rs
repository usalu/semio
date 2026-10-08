//! 🔺️ Diff for `RemoveEdge`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveEdge, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::RemoveEdge { id } = payload;
    protocol::MutationOutcome::new(diff_remove_edge(id))
}
//#endregion 🔖️Diff
