//! ↩️ Inverse for `ChangeTargetVolumeHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeTargetVolumeHidden, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::change_target_volume_hidden(item.id.clone(), item.hidden)]
}
//#endregion 🔖️Inverse
