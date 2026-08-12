//! 🔺️ Sparse diff builder for `ReplacePart2dGeometry` — patches the one addressed part in place.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplacePart2dGeometry, base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle5dDiff::default();
    };
    let mut next = item.clone();
    next.part_2d.shape = payload.new_shape.clone();
    next.part_2d.radius = payload.new_radius;
    next.part_2d.width = payload.new_width;
    next.part_2d.height = payload.new_height;
    Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
