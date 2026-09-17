//! ↩️ Inverse for `MoveTargetRegion` — restores the BASE field values on the addressed region. Missing
//! target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::MoveTargetRegion, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    let Some(region) = base.target_regions.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::move_target_region::move_target_region(region.id.clone(), region.x, region.y)]
}
//#endregion 🔖️Inverse
