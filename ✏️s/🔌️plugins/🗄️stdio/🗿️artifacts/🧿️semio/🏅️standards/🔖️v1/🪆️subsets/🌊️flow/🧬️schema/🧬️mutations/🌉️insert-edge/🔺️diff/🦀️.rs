//! 🔺️ Diff for `InsertEdge`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertEdge, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    if base.edges.iter().any(|e| e.id == payload.edge.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Edge \"{}\" already exists.", payload.edge.id), [payload.edge.id.clone()]);
    }
    let super::InsertEdge { edge, at } = payload;
    protocol::MutationOutcome::new(diff_insert_edge(base, edge.clone(), *at))
}
//#endregion 🔖️Diff
