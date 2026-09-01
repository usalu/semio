//! 🔺️ Sparse diff builder for `RotateTargetVolume` — patches the one addressed target-volume in place.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumePatchEntry, Puzzle3dTargetVolumesDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RotateTargetVolume, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target-volume", payload.id), vec![payload.id.clone()]);
    };
    let mut next = item.clone();
    next.orientation = payload.new_orientation;
    if next == *item {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        target_volumes: Some(Puzzle3dTargetVolumesDelta { patched: vec![Puzzle3dTargetVolumePatchEntry { id: payload.id.clone(), patch: Puzzle3dTargetVolumePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
