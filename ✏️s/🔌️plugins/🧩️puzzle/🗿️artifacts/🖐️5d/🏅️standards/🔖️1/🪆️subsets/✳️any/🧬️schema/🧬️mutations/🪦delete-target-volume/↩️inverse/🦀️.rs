//! ↩️ Inverse for `DeleteTargetVolume` — reconstructs a `create-target-volume` of the captured BASE
//! entry AT ITS OWN INDEX, so undoing a delete restores the order too. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteTargetVolume, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(index) = base.target_volumes.iter().position(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::create_target_volume::create_target_volume(base.target_volumes[index].clone(), Some(index))]
}
//#endregion 🔖️Inverse
