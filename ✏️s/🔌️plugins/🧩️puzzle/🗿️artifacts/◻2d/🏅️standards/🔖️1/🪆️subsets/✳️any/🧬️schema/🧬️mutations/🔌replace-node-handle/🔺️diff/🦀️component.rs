//! 🔺️ Sparse diff builder for `ReplaceNodeHandle` — patches one handle inside the owner node.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.clone()]);
    };
    if !node.handles.iter().any(|handle| handle.id == payload.handle_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Handle \"{}\" not found on node \"{}\".", payload.handle_id, payload.node_id), vec![payload.handle_id.clone()]);
    }
    let mut next = node.clone();
    if next == *node {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.node_id.clone()])]);
    }
    for handle in next.handles.iter_mut() {
        if handle.id == payload.handle_id {
            *handle = payload.new_handle.clone();
        }
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.node_id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
