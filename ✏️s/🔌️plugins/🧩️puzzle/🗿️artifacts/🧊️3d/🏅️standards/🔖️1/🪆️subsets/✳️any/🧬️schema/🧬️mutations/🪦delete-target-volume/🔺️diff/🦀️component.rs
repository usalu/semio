//! 🔺️ Sparse diff builder for `DeleteTargetVolume` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle3d::diff::{Puzzle3dTargetVolumesDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteTargetVolume, _base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    Puzzle3dDiff { target_volumes: Some(Puzzle3dTargetVolumesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
