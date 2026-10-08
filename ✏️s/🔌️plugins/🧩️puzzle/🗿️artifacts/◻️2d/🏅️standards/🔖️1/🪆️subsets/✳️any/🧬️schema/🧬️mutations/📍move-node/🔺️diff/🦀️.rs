//! 🔺️ Sparse diff builder for `MoveNode` — patches the one addressed node's position.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_finite;

//#region 🔖️Diff
pub fn diff(payload: &super::MoveNode, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_finite(&[("newX", payload.new_x), ("newY", payload.new_y)]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.to_string_owner()]);
    }
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dNodePatch {
        x: (payload.new_x != node.x).then_some(payload.new_x),
        y: (payload.new_y != node.y).then_some(payload.new_y),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.to_string_owner()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
