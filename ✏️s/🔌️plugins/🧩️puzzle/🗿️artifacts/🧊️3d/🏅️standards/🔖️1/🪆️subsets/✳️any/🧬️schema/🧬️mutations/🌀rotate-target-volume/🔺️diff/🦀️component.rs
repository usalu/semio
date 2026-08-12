//! 🔺️ Sparse diff builder for `RotateTargetVolume` — patches the one addressed target-volume in place.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolumePatchEntry, Puzzle3dTargetVolumesDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RotateTargetVolume, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Puzzle3dDiff::default();
    };
    let mut next = item.clone();
    next.orientation = payload.new_orientation;
    Puzzle3dDiff {
        target_volumes: Some(Puzzle3dTargetVolumesDelta { patched: vec![Puzzle3dTargetVolumePatchEntry { id: payload.id.clone(), patch: Puzzle3dTargetVolumePatch { replacement: Some(next) } }], ..Default::default() }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
