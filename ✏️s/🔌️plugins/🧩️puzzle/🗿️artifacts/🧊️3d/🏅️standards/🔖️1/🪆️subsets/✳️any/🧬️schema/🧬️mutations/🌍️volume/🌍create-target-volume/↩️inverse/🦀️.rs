//! ↩️ Inverse for `CreateTargetVolume` — always a `delete-target-volume` of the id it created.
use super::mutation::CreateTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::DeleteTargetVolume;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateTargetVolume> + From<DeleteTargetVolume>>(payload: &super::mutation::CreateTargetVolume, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::mutation::delete_target_volume(payload.target_volume.id.clone())]

    })())
}
//#endregion 🔖️Inverse
