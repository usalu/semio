//! 🔺️ Sparse diff builder for `ReplaceNodeGeometry` — patches the one addressed node's shape/extent.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_positive, puzzle2d_shape};

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceNodeGeometry, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_shape("newShape", payload.new_shape.as_deref()).and_then(|()| puzzle2d_positive(&[("newRadius", payload.new_radius), ("newWidth", payload.new_width), ("newHeight", payload.new_height)])) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.clone()]);
    }
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.clone()]);
    };
    let mut next = node.clone();
    next.shape = payload.new_shape.clone();
    next.radius = payload.new_radius;
    next.width = payload.new_width;
    next.height = payload.new_height;
    if next == *node {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: Some(Puzzle2dNodesDelta { patched: vec![Puzzle2dNodePatchEntry { id: payload.id.clone(), patch: Puzzle2dNodePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
