//! 🔺️ Sparse diff builder for `AddPartGrip` — adds one grip to the owner part's `grips`. No-op when the
//! grip id already exists on that part.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dGripsDelta, Puzzle5dPartPatch, Puzzle5dPartsDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddPartGrip, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part-grip", payload.part_id), vec![payload.part_id.clone()]);
    };
    if part.grips.iter().any(|grip| grip.id == payload.grip.id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Grip \"{}\" already exists on part \"{}\".", payload.grip.id, payload.part_id));
    }
    let index = payload.index.map_or(part.grips.len(), |index| index.min(part.grips.len()));
    let patch = Puzzle5dPartPatch { grips: Some(Puzzle5dGripsDelta::insertion(index, payload.grip.clone())), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle5dDiff { parts: Some(Puzzle5dPartsDelta::modification(payload.part_id.clone(), patch)), ..Default::default() })
}
//#endregion 🔖️Diff
