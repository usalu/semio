//! 🔺️ Diff for `SetEdgeEndpoints`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEdgeEndpoints, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::SetEdgeEndpoints { id, from, to } = payload;
    protocol::MutationOutcome::new(diff_set_edge_endpoints(id, from.clone(), to.clone()))
}
//#endregion 🔖️Diff
