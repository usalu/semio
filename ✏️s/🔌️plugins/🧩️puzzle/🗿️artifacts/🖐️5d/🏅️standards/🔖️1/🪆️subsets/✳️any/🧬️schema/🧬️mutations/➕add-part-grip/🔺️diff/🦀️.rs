//! 🔺️ Sparse diff builder for `AddPartGrip` — patches the owner part's `grips` list. No-op when the
//! grip id already exists on that part.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddPartGrip, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part-grip", payload.part_id), vec![payload.part_id.clone()]);
    };
    if part.grips.iter().any(|grip| grip.id == payload.grip.id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Grip \"{}\" already exists on part \"{}\".", payload.grip.id, payload.part_id));
    }
    let mut next = part.clone();
    let at = payload.index.unwrap_or(next.grips.len()).min(next.grips.len());
    next.grips.insert(at, payload.grip.clone());
    if next == *part {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.part_id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.part_id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
