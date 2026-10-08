//! 🔺️ Diff for `RemoveNodeParam`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveNodeParam, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    if !base.nodes.iter().any(|n| n.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !base.nodes.iter().find(|n| n.id == payload.id).is_some_and(|n| n.params.iter().any(|p| p.key == payload.key)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" has no parameter \"{}\".", payload.id, payload.key), [payload.id.clone(), payload.key.clone()]);
    }
    let super::RemoveNodeParam { id, key } = payload;
    protocol::MutationOutcome::new(diff_remove_node_param(id, key))
}
//#endregion 🔖️Diff
