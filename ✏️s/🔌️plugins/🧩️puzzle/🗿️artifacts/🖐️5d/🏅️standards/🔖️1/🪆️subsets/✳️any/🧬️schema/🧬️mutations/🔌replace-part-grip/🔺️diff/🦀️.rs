//! 🔺️ Sparse diff builder for `ReplacePartGrip` — patches one grip inside the owner part.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplacePartGrip, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part-grip", payload.part_id), vec![payload.part_id.clone()]);
    };
    if !part.grips.iter().any(|grip| grip.id == payload.grip_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Grip \"{}\" not found on part \"{}\".", payload.grip_id, payload.part_id), vec![payload.grip_id.clone()]);
    }
    let mut next = part.clone();
    for grip in next.grips.iter_mut() {
        if grip.id == payload.grip_id {
            *grip = payload.new_grip.clone();
        }
    }
    if next == *part {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.part_id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.part_id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
