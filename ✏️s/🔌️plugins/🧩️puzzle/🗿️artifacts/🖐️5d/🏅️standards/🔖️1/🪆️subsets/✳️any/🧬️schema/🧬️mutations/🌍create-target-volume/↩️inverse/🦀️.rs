//! ↩️ Inverse for `CreateTargetVolume` — always a `delete-target-volume` of the id it created.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::CreateTargetVolume, _base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::DeleteTargetVolume>,
{
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::delete_target_volume(payload.target_volume.id.clone())]

    })())
}
//#endregion 🔖️Inverse
