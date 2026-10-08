//! 🔺️ Diff for `SetNodeKind`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetNodeKind, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    if !base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.nodes.iter().find(|n| n.id == payload.id).is_some_and(|n| n.kind == payload.kind) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The node already has that kind.");
    }
    let super::SetNodeKind { id, kind } = payload;
    protocol::MutationOutcome::new(diff_set_node_kind(id, kind))
}
//#endregion 🔖️Diff
