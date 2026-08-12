//! 🔺️ Sparse diff builder for `ReplacePartGrip` — patches one grip inside the owner part.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplacePartGrip, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return Puzzle5dDiff::default();
    };
    if !part.grips.iter().any(|grip| grip.id == payload.grip_id) {
        return Puzzle5dDiff::default();
    }
    let mut next = part.clone();
    for grip in next.grips.iter_mut() {
        if grip.id == payload.grip_id {
            *grip = payload.new_grip.clone();
        }
    }
    Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.part_id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
