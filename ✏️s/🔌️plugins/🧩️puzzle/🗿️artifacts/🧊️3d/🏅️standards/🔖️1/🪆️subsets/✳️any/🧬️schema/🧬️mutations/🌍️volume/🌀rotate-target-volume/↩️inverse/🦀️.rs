//! ↩️ Inverse for `RotateTargetVolume` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::RotateTargetVolume;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<RotateTargetVolume>>(payload: &super::mutation::RotateTargetVolume, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::mutation::rotate_target_volume(item.id.clone(), item.orientation)]

    })())
}
//#endregion 🔖️Inverse
