//! target ⇒ `Vec::new()`.
//! ↩️ Inverse for `ChangeTargetRegionLocked` — restores the BASE field values on the addressed region. Missing
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeTargetRegionLocked>>(payload: &super::ChangeTargetRegionLocked, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::change_target_region_locked::change_target_region_locked(region.id.clone(), region.locked)]

    })())
}
//#endregion 🔖️Inverse
