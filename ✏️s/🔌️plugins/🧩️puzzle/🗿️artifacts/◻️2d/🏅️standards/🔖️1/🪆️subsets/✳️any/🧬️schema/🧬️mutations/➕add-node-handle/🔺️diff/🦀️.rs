//! 🔺️ Sparse diff builder for `AddNodeHandle` — adds one handle to the owner node's `🐙️handles`. No-op
//! when the handle id already exists on that node.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dHandlesDelta, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_handle_invariant;

//#region 🔖️Diff
pub fn diff(payload: &super::AddNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_handle_invariant(&payload.handle) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.node_id.to_string_owner(), payload.handle.id.to_string_owner()]);
    }
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.to_string_owner()]);
    };
    if node.handles.iter().any(|handle| handle.id == payload.handle.id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Handle \"{}\" already exists on node \"{}\".", payload.handle.id, payload.node_id));
    }
    let reordered = payload.index.filter(|index| *index < node.handles.len()).map(|index| {
        let mut order: Vec<_> = node.handles.iter().map(|handle| handle.id.clone()).collect();
        order.insert(index, payload.handle.id.clone());
        order
    });
    let patch = Puzzle2dNodePatch { handles: Some(Puzzle2dHandlesDelta::adding(payload.handle.clone(), reordered)), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle2dDiff { nodes: Some(Puzzle2dNodesDelta::patching(payload.node_id.clone(), patch)), ..Default::default() })
}
//#endregion 🔖️Diff
