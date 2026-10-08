//! 🔺️ Sparse diff builder for `RemovePartGrip` — removes one grip from the owner part's `grips` and severs
//! any fastener referencing the removed grip (full id `part_id:grip_id`).
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta, Puzzle5dGripsDelta, Puzzle5dPartPatch, Puzzle5dPartsDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemovePartGrip, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part-grip", payload.part_id), vec![payload.part_id.clone()]);
    };
    if !part.grips.iter().any(|grip| grip.id == payload.grip_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Grip \"{}\" not found on part \"{}\".", payload.grip_id, payload.part_id), vec![payload.grip_id.clone()]);
    }
    let full_id = format!("{}:{}", payload.part_id, payload.grip_id);
    let severed: Vec<String> = base.fasteners.iter().filter(|fastener| fastener.source == full_id || fastener.target == full_id).map(|fastener| fastener.id.clone()).collect();
    let patch = Puzzle5dPartPatch { grips: Some(Puzzle5dGripsDelta::removing(vec![payload.grip_id.clone()])), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta::patching(payload.part_id.clone(), patch)),
        fasteners: (!severed.is_empty()).then(|| Puzzle5dFastenersDelta::removing(severed)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
