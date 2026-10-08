//! ↩️ Inverse for `DeleteTargetRegion` — reconstructs a `create-target-region` of the captured BASE
//! entry. Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteTargetRegion, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.target_regions.iter().position(|entry| entry.id == payload.id);
    vec![crate::standards::v1::subsets::any::schema::mutations::create_target_region::create_target_region(region.clone(), index)]

    })())
}
//#endregion 🔖️Inverse
