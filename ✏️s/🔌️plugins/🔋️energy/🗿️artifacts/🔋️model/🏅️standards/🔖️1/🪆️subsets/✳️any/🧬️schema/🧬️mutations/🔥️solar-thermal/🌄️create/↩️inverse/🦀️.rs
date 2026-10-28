//! ↩️ Inverse for `CreateSolarThermalSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateSolarThermalSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateSolarThermalSystem> + From<crate::DeleteSolarThermalSystem>,
{
    Ok((|| {
    if base.model.solar_thermal_systems.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.solar_thermal_systems.len() {
        return Vec::new();
    }
    vec![vocabulary::delete_solar_thermal_system(payload.id)]

    })())
}
//#endregion 🔖️Inverse
