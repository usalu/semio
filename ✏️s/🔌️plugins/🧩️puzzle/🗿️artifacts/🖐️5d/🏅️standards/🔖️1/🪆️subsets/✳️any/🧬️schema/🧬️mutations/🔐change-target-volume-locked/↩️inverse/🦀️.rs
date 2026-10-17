//! ↩️ Inverse for `ChangeTargetVolumeLocked` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ChangeTargetVolumeLocked, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::ChangeTargetVolumeLocked>,
{
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::change_target_volume_locked(item.id.clone(), item.locked)]

    })())
}
//#endregion 🔖️Inverse
