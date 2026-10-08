//! ↩️ Inverse for `DeleteTargetVolume` — reconstructs a `create-target-volume` of the captured
//! BASE entry. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteTargetVolume, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.target_volumes.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.target_volumes.iter().position(|entry| entry.id == payload.id);
    vec![crate::standards::v1::subsets::any::schema::mutations::create_target_volume::mutation::create_target_volume(item.clone(), index)]

    })())
}
//#endregion 🔖️Inverse
