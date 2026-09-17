//! ↩️ Inverse for `CreateTargetVolume` — always a `delete-target-volume` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateTargetVolume, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::delete_target_volume(payload.target_volume.id.clone())]
}
//#endregion 🔖️Inverse
