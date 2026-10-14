//! ↩️ Inverse for `ChangeTargetVolumeHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ChangeTargetVolumeHidden;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeTargetVolumeHidden>>(payload: &super::mutation::ChangeTargetVolumeHidden, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::mutation::change_target_volume_hidden(item.id.clone(), item.hidden)]

    })())
}
//#endregion 🔖️Inverse
