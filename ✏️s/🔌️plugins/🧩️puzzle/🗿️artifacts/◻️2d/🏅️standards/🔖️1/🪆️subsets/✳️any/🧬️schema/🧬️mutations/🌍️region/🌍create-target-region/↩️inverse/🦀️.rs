//! ↩️ Inverse for `CreateTargetRegion` — always a `delete-target-region` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::delete_target_region::DeleteTargetRegion;
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::create_target_region::component::CreateTargetRegion;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateTargetRegion> + From<DeleteTargetRegion>>(payload: &super::CreateTargetRegion, _base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_target_region::delete_target_region(payload.target_region.id.clone())]

    })())
}
//#endregion 🔖️Inverse
