//! 🔺️ Sparse diff builder for `RemovePartGrip` — patches the owner part's `grips` list and severs
//! any fastener referencing the removed grip (full id `part_id:grip_id`).
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta, Puzzle5dPartPatch, Puzzle5dPartPatchEntry, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemovePartGrip, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part-grip", payload.part_id), vec![payload.part_id.clone()]);
    };
    if !part.grips.iter().any(|grip| grip.id == payload.grip_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Grip \"{}\" not found on part \"{}\".", payload.grip_id, payload.part_id), vec![payload.grip_id.clone()]);
    }
    let mut next = part.clone();
    next.grips.retain(|grip| grip.id != payload.grip_id);
    let full_id = format!("{}:{}", payload.part_id, payload.grip_id);
    let severed: Vec<String> = base.fasteners.iter().filter(|fastener| fastener.source == full_id || fastener.target == full_id).map(|fastener| fastener.id.clone()).collect();
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta { patched: vec![Puzzle5dPartPatchEntry { id: payload.part_id.clone(), patch: Puzzle5dPartPatch { replacement: Some(next) } }], ..Default::default() }),
        fasteners: if severed.is_empty() { None } else { Some(Puzzle5dFastenersDelta { removed: severed, ..Default::default() }) },
        ..Default::default()
    })
}
//#endregion 🔖️Diff
