//! 🔺️ Sparse diff builder for `ReplaceNodeGeometry` — patches the one addressed node's shape/extent.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle2dDiff, Puzzle2dNodePatch, Puzzle2dNodesDelta};
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_positive,puzzle2d_shape};


//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceNodeGeometry, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_shape("newShape", payload.new_shape.as_ref()).and_then(|()| puzzle2d_positive(&[("newRadius", payload.new_radius), ("newWidth", payload.new_width), ("newHeight", payload.new_height)])) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.to_string_owner()]);
    }
    let Some(node) = base.nodes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "node", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dNodePatch {
        shape: (payload.new_shape != node.shape).then(|| payload.new_shape.clone()),
        radius: (payload.new_radius != node.radius).then_some(payload.new_radius),
        width: (payload.new_width != node.width).then_some(payload.new_width),
        height: (payload.new_height != node.height).then_some(payload.new_height),
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
