//! ↩️ Inverse for `DeleteSolarThermalSystem` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteSolarThermalSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.solar_thermal_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.solar_thermal_systems[index];
    vec![vocabulary::create_solar_thermal_system(index as u32, existing.id, existing.collector_area_m2, existing.efficiency, existing.storage_volume_m3, existing.tilt_deg, existing.azimuth_deg)]
}
//#endregion 🔖️Inverse
