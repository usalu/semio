//! 🔺️ Diff for `SetEdgeEndpoints`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEdgeEndpoints, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    if !base.edges.iter().any(|e| e.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.edges.iter().find(|e| e.id == payload.id).is_some_and(|e| e.from == payload.from && e.to == payload.to) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The edge already has those endpoints.");
    }
    let super::SetEdgeEndpoints { id, from, to } = payload;
    protocol::MutationOutcome::new(diff_set_edge_endpoints(id, from.clone(), to.clone()))
}
//#endregion 🔖️Diff
