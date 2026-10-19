//! 🫥️ entry. Missing target ⇒ `Vec::new()`.
//! ↩️ Inverse for `DeleteTargetRegion` — reconstructs a `create-target-region` of the captured BASE
use crate::standards::v1::subsets::any::schema::mutations::create_target_region::CreateTargetRegion;
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::delete_target_region::component::DeleteTargetRegion;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteTargetRegion> + From<CreateTargetRegion>>(payload: &super::DeleteTargetRegion, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.target_regions.iter().position(|entry| entry.id == payload.id);
    vec![crate::standards::v1::subsets::any::schema::mutations::create_target_region::create_target_region(region.clone(), index)]

    })())
}
//#endregion 🔖️Inverse
