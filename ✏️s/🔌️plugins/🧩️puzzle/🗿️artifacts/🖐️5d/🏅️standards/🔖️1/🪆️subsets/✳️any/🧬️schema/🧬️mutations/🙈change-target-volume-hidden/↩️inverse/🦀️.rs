//! ↩️ Inverse for `ChangeTargetVolumeHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ChangeTargetVolumeHidden, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::ChangeTargetVolumeHidden>,
{
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::change_target_volume_hidden(item.id.clone(), item.hidden)]

    })())
}
//#endregion 🔖️Inverse
