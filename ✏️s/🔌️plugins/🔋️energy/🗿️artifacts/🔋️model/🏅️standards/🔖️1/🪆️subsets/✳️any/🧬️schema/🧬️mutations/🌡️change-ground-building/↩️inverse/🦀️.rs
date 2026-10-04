//! ↩️ Inverse for `ChangeGroundBuilding` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeGroundBuilding, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if !((1..=12).contains(&payload.month) && payload.new_temperature_c.is_finite() && payload.new_temperature_c >= -273.15) || base.model.ground_temperature.building_surface_c[usize::from(payload.month) - 1] == payload.new_temperature_c {
        return Vec::new();
    }
    vec![vocabulary::change_ground_building(payload.month, base.model.ground_temperature.building_surface_c[usize::from(payload.month) - 1])]

    })())
}
//#endregion 🔖️Inverse
