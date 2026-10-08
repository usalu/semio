//! 🔺️ Sparse diff builder for `ReplaceFastenerGeometry` — patches the one addressed fastener in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dFastenerPatch, Puzzle5dFastenersDelta};
use protocol::list_delta::RowPatch;
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceFastenerGeometry, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.fasteners.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "fastener", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle5dFastenerPatch {
        gap: (payload.new_gap != item.gap).then_some(payload.new_gap),
        shift: (payload.new_shift != item.shift).then_some(payload.new_shift),
        rise: (payload.new_rise != item.rise).then_some(payload.new_rise),
        rotation: (payload.new_rotation != item.rotation).then_some(payload.new_rotation),
        turn: (payload.new_turn != item.turn).then_some(payload.new_turn),
        tilt: (payload.new_tilt != item.tilt).then_some(payload.new_tilt),
        x: (payload.new_x != item.x).then_some(payload.new_x),
        y: (payload.new_y != item.y).then_some(payload.new_y),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        fasteners: Some(Puzzle5dFastenersDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
