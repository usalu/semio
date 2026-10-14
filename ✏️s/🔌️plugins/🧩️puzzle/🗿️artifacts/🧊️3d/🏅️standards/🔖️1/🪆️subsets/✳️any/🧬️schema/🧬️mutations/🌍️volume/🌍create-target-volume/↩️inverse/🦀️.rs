//! ↩️ Inverse for `CreateTargetVolume` — always a `delete-target-volume` of the id it created.
use super::mutation::CreateTargetVolume;
use crate::delete_target_volume::mutation::DeleteTargetVolume;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateTargetVolume> + From<DeleteTargetVolume>>(payload: &super::mutation::CreateTargetVolume, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::delete_target_volume::mutation::delete_target_volume(payload.target_volume.id.clone())]

    })())
}
//#endregion 🔖️Inverse
