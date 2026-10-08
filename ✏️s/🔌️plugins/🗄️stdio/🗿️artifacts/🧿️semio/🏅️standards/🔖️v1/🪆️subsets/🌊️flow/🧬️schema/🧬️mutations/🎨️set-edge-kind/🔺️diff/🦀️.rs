//! 🔺️ Diff for `SetEdgeKind`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEdgeKind, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::SetEdgeKind { id, kind } = payload;
    protocol::MutationOutcome::new(diff_set_edge_kind(id, kind))
}
//#endregion 🔖️Diff
