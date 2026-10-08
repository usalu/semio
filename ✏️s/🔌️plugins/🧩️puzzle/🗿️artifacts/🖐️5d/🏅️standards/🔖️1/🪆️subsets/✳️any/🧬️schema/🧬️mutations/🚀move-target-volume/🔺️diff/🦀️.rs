//! 🔺️ Sparse diff builder for `MoveTargetVolume` — patches the one addressed target-volume in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle5dDiff, Puzzle5dTargetVolumePatch, Puzzle5dTargetVolumesDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::MoveTargetVolume, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target-volume", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle5dTargetVolumePatch {
        origin: (payload.new_origin != item.origin).then_some(payload.new_origin),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        target_volumes: Some(Puzzle5dTargetVolumesDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
