//! 🔺️ Sparse diff builder for `MoveNode` — patches the one addressed node's position.
use crate::artifacts::puzzle2d::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MoveNode, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.clone()]);
    };
    let mut next = node.clone();
    next.x = payload.new_x;
    next.y = payload.new_y;
    if next == *node {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
