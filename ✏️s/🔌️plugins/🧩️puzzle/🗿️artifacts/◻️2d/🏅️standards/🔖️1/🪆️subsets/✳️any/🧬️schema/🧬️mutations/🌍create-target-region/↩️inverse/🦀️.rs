//! ↩️ Inverse for `CreateTargetRegion` — always a `delete-target-region` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateTargetRegion, _base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_target_region::delete_target_region(payload.target_region.id.clone())]
}
//#endregion 🔖️Inverse
