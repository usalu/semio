//! 🔺️ Sparse diff builder for `ReplaceNodeHandle` — patches one handle inside the owner node. An
//! absent node or an absent handle is `mutation.target-missing`; the `mutation.no-op` warning is
//! reserved for a replacement that is equal to the handle it replaces, which is why the guard runs
//! AFTER the replacement loop rather than before it.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.clone()]);
    };
    if !node.handles.iter().any(|handle| handle.id == payload.handle_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Handle \"{}\" not found on node \"{}\".", payload.handle_id, payload.node_id), vec![payload.handle_id.clone()]);
    }
    let mut next = node.clone();
    for handle in next.handles.iter_mut() {
        if handle.id == payload.handle_id {
            *handle = payload.new_handle.clone();
        }
    }
    if next == *node {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.node_id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.node_id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
