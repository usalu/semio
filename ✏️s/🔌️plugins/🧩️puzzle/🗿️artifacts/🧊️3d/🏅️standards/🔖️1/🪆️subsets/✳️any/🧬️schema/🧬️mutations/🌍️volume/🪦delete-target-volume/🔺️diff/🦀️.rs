//! 🔺️ Sparse diff builder for `DeleteTargetVolume` — a real removal, never a whole-snapshot capture.
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dTargetVolumesDelta;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteTargetVolume, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(at) = base.target_volumes.iter().position(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "target volume", payload.id), vec![payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Puzzle3dDiff { target_volumes: Some(Puzzle3dTargetVolumesDelta::removal_by_id(payload.id.clone(), at)), ..Default::default() })
}
//#endregion 🔖️Diff
