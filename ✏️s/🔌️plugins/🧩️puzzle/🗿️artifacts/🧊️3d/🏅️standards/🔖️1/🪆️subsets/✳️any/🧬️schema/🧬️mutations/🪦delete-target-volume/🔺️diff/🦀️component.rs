//! 🔺️ Sparse diff builder for `DeleteTargetVolume` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle3d::diff::{Puzzle3dTargetVolumesDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteTargetVolume, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !base.target_volumes.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target volume", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff { target_volumes: Some(Puzzle3dTargetVolumesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
