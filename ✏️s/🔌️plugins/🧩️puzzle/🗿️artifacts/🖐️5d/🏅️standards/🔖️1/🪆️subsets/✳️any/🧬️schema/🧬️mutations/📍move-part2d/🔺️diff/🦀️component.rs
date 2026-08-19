//! 🔺️ Sparse diff builder for `MovePart2d` — patches the one addressed part in place.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MovePart2d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part", payload.id), vec![payload.id.clone()]);
    };
    let mut next = item.clone();
    next.part_2d.x = payload.new_x;
    next.part_2d.y = payload.new_y;
    if next == *item {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
