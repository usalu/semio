//! ↩️ Inverse for `ScaleTargetVolume` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ScaleTargetVolume, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::ScaleTargetVolume>,
{
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::scale_target_volume(item.id.clone(), item.scale)]

    })())
}
//#endregion 🔖️Inverse
