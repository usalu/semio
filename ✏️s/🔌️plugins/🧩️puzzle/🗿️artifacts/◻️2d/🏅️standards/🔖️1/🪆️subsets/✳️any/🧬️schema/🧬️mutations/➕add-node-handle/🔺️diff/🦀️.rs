//! 🔺️ Sparse diff builder for `AddNodeHandle` — patches the owner node's `🐙️handles` list. No-op
//! when the handle id already exists on that node.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.clone()]);
    };
    if node.handles.iter().any(|handle| handle.id == payload.handle.id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Handle \"{}\" already exists on node \"{}\".", payload.handle.id, payload.node_id));
    }
    let mut next = node.clone();
    let at = payload.index.unwrap_or(next.handles.len()).min(next.handles.len());
    next.handles.insert(at, payload.handle.clone());
    if next == *node {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.node_id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.node_id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
