//! 🔺️ Diff for `SetEdgeKind`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetEdgeKind, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    if !base.edges.iter().any(|e| e.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.edges.iter().find(|e| e.id == payload.id).is_some_and(|e| e.kind == payload.kind) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The edge already has that kind.");
    }
    let super::SetEdgeKind { id, kind } = payload;
    protocol::MutationOutcome::new(diff_set_edge_kind(id, kind))
}
//#endregion 🔖️Diff
