//! 🔺️ Sparse diff builder for `ReplaceFastenerGeometry` — patches the one addressed fastener in place.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dFastenerPatch, Puzzle5dFastenerPatchEntry, Puzzle5dFastenersDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceFastenerGeometry, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.fasteners.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "fastener", payload.id), vec![payload.id.clone()]);
    };
    let mut next = item.clone();
    next.gap = payload.new_gap;
    next.shift = payload.new_shift;
    next.rise = payload.new_rise;
    next.rotation = payload.new_rotation;
    next.turn = payload.new_turn;
    next.tilt = payload.new_tilt;
    next.x = payload.new_x;
    next.y = payload.new_y;
    if next == *item {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        fasteners: Some(Puzzle5dFastenersDelta { patched: vec![Puzzle5dFastenerPatchEntry { id: payload.id.clone(), patch: Puzzle5dFastenerPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
