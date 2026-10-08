//! 🔺️ Sparse diff builder for `ScaleNode` — patches the one addressed node in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_positive;

//#region 🔖️Diff
pub fn diff(payload: &super::ScaleNode, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_positive(&[("newScale", payload.new_scale)]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.to_string_owner()]);
    }
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dNodePatch {
        scale: (payload.new_scale != node.scale).then_some(payload.new_scale),
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
