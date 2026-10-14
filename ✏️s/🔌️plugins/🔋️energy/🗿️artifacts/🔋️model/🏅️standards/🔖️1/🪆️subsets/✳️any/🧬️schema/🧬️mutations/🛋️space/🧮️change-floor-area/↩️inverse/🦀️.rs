//! ↩️ Inverse for `ChangeSpaceFloorArea` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSpaceFloorArea, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSpaceFloorArea>,
{
    Ok((|| {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_floor_area_m2.is_finite() || payload.new_floor_area_m2 < 0.0 || existing.floor_area_m2 == payload.new_floor_area_m2 {
        return Vec::new();
    }
    vec![vocabulary::change_space_floor_area(payload.id, existing.floor_area_m2)]

    })())
}
//#endregion 🔖️Inverse
