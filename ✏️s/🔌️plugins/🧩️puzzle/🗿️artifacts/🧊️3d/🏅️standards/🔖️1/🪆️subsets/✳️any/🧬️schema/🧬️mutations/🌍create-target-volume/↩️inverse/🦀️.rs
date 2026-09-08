//! ↩️ Inverse for `CreateTargetVolume` — always a `delete-target-volume` of the id it created.
use crate::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateTargetVolume, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::mutations::delete_target_volume::mutation::delete_target_volume(payload.target_volume.id.clone())]
}
//#endregion 🔖️Inverse
