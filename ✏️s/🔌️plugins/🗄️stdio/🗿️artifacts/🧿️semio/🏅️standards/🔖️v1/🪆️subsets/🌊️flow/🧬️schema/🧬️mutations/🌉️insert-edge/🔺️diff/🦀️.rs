//! 🔺️ Diff for `InsertEdge`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertEdge, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    let super::InsertEdge { edge, at } = payload;
    protocol::MutationOutcome::new(diff_insert_edge(base, edge.clone(), *at))
}
//#endregion 🔖️Diff
