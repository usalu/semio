//! ↩️ Inverse for `CreateTargetVolume` — always a `delete-target-volume` of the id it created.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateTargetVolume, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::artifacts::puzzle3d::mutations::delete_target_volume::mutation::delete_target_volume(payload.target_volume.id.clone())]
}
//#endregion 🔖️Inverse
