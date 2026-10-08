//! 🔺️ Diff for `InsertNode`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertNode, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
    if base.nodes.iter().any(|n| n.id == payload.node.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Node \"{}\" already exists.", payload.node.id), [payload.node.id.clone()]);
    }
    let super::InsertNode { node, at } = payload;
    protocol::MutationOutcome::new(diff_insert_node(base, node.clone(), *at))
}
//#endregion 🔖️Diff
