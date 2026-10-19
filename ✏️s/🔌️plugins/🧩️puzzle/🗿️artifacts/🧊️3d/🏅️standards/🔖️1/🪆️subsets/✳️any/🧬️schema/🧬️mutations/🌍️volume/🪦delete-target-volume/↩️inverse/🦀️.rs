//! 🫥️ BASE entry. Missing target ⇒ `Vec::new()`.
//! ↩️ Inverse for `DeleteTargetVolume` — reconstructs a `create-target-volume` of the captured
use super::mutation::DeleteTargetVolume;
use crate::create_target_volume::mutation::CreateTargetVolume;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteTargetVolume> + From<CreateTargetVolume>>(payload: &super::mutation::DeleteTargetVolume, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.target_volumes.iter().position(|entry| entry.id == payload.id);
    vec![crate::create_target_volume::mutation::create_target_volume(item.clone(), index)]

    })())
}
//#endregion 🔖️Inverse
