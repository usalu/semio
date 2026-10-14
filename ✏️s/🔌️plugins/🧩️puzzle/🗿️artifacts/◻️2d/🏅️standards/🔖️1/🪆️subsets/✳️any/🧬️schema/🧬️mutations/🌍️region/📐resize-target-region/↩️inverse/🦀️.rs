//! ↩️ Inverse for `ResizeTargetRegion` — restores the BASE field values on the addressed region. Missing
//! target ⇒ `Vec::new()`.
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ResizeTargetRegion>>(payload: &super::ResizeTargetRegion, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::resize_target_region::resize_target_region(region.id.clone(), region.width, region.height)]

    })())
}
//#endregion 🔖️Inverse
