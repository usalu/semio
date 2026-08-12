//! 🔺️ Sparse diff builder for `ChangePart2dIcon` — patches the one addressed part in place.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangePart2dIcon, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle5dDiff::default();
    };
    let mut next = item.clone();
    next.part_2d.icon_kind = payload.new_icon_kind.clone();
    Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
