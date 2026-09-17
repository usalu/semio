//! ↩️ Inverse for `DeleteTargetVolume` — reconstructs a `create-target-volume` of the captured BASE
//! entry. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteTargetVolume, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::create_target_volume::create_target_volume(item.clone(), None)]
}
//#endregion 🔖️Inverse
