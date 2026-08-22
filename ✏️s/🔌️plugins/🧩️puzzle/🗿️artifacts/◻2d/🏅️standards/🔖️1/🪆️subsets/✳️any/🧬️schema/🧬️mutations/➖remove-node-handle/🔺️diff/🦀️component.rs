//! 🔺️ Sparse diff builder for `RemoveNodeHandle` — patches the owner node's `handles` list and
//! severs any edge referencing the removed handle.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveNodeHandle, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node-handle", payload.node_id), vec![payload.node_id.clone()]);
    };
    if !node.handles.iter().any(|handle| handle.id == payload.handle_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Handle \"{}\" not found on node \"{}\".", payload.handle_id, payload.node_id), vec![payload.handle_id.clone()]);
    }
    let mut next = node.clone();
    next.handles.retain(|handle| handle.id != payload.handle_id);
    let severed: Vec<String> = base.edges.iter().filter(|edge| edge.source == payload.handle_id || edge.target == payload.handle_id).map(|edge| edge.id.clone()).collect();
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.node_id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        edges: if severed.is_empty() { None } else { Some(Puzzle2dEdgesDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
