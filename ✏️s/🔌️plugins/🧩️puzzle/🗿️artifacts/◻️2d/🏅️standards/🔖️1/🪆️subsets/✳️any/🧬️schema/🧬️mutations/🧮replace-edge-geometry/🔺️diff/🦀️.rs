//! 🔺️ Sparse diff builder for `ReplaceEdgeGeometry` — patches the one addressed edge's connection
//! pose.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgePatch, Puzzle2dEdgesDelta};
use protocol::list_delta::RowPatch;
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_finite;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceEdgeGeometry, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_finite(&[("newGap", payload.new_gap), ("newShift", payload.new_shift), ("newRise", payload.new_rise), ("newRotation", payload.new_rotation), ("newTurn", payload.new_turn), ("newTilt", payload.new_tilt), ("newX", payload.new_x), ("newY", payload.new_y)]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.to_string_owner()]);
    }
    let Some(edge) = base.edges.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "edge", payload.id), vec![payload.id.to_string_owner()]);
    };
    let patch = Puzzle2dEdgePatch {
        gap: (payload.new_gap != edge.gap).then_some(payload.new_gap),
        shift: (payload.new_shift != edge.shift).then_some(payload.new_shift),
        rise: (payload.new_rise != edge.rise).then_some(payload.new_rise),
        rotation: (payload.new_rotation != edge.rotation).then_some(payload.new_rotation),
        turn: (payload.new_turn != edge.turn).then_some(payload.new_turn),
        tilt: (payload.new_tilt != edge.tilt).then_some(payload.new_tilt),
        x: (payload.new_x != edge.x).then_some(payload.new_x),
        y: (payload.new_y != edge.y).then_some(payload.new_y),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.to_string_owner()])]);
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        edges: Some(Puzzle2dEdgesDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
