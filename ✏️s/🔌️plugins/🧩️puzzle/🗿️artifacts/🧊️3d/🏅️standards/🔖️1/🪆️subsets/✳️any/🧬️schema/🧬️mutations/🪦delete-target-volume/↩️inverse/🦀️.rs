//! ↩️ Inverse for `DeleteTargetVolume` — reconstructs a `create-target-volume` of the captured
//! BASE entry. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteTargetVolume, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::create_target_volume::mutation::create_target_volume(item.clone(), None)]
}
//#endregion 🔖️Inverse
