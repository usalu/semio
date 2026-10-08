//! 🔺️ Diff for `SetNodeKind`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetNodeKind, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::SetNodeKind { id, kind } = payload;
    protocol::MutationOutcome::new(diff_set_node_kind(id, kind))
}
//#endregion 🔖️Diff
