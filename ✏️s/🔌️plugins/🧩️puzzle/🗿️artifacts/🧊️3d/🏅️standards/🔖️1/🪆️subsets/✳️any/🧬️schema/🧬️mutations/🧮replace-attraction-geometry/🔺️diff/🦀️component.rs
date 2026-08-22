//! 🔺️ Sparse diff builder for `ReplaceAttractionGeometry` — patches the one addressed attraction in place.
use crate::artifacts::puzzle3d::diff::{Puzzle3dAttractionPatch, Puzzle3dAttractionPatchEntry, Puzzle3dAttractionsDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceAttractionGeometry, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.attractions.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attraction", payload.id), vec![payload.id.clone()]);
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
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        attractions: Some(Puzzle3dAttractionsDelta { patched: vec![Puzzle3dAttractionPatchEntry { id: payload.id.clone(), patch: Puzzle3dAttractionPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
