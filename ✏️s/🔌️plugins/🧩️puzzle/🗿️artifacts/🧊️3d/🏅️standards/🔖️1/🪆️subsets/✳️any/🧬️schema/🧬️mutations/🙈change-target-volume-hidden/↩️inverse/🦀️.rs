//! ↩️ Inverse for `ChangeTargetVolumeHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeTargetVolumeHidden, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::change_target_volume_hidden::mutation::change_target_volume_hidden(item.id.clone(), item.hidden)]
}
//#endregion 🔖️Inverse
