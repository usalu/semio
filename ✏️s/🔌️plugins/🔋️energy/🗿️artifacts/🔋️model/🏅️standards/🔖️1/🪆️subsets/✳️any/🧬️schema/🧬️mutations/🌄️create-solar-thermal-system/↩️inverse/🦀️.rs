//! ↩️ Inverse for `CreateSolarThermalSystem` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateSolarThermalSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.solar_thermal_systems.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.solar_thermal_systems.len() {
        return Vec::new();
    }
    vec![vocabulary::delete_solar_thermal_system(payload.id)]
}
//#endregion 🔖️Inverse
