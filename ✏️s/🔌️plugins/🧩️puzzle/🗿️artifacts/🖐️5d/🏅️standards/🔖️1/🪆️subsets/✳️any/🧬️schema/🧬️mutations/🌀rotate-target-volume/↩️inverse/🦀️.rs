//! ↩️ Inverse for `RotateTargetVolume` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::RotateTargetVolume, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::RotateTargetVolume>,
{
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::rotate_target_volume(item.id.clone(), item.orientation)]

    })())
}
//#endregion 🔖️Inverse
