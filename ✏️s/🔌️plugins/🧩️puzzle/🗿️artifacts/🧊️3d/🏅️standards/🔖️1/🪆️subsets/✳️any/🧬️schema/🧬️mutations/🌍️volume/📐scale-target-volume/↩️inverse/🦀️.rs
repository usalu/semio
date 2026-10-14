//! ↩️ Inverse for `ScaleTargetVolume` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ScaleTargetVolume;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ScaleTargetVolume>>(payload: &super::mutation::ScaleTargetVolume, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::mutation::scale_target_volume(item.id.clone(), item.scale)]

    })())
}
//#endregion 🔖️Inverse
