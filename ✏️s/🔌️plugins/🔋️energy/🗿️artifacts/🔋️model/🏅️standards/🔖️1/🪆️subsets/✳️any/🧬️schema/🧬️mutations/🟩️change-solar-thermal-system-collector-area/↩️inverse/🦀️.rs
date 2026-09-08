//! ↩️ Inverse for `ChangeSolarThermalSystemCollectorArea` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSolarThermalSystemCollectorArea, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.collector_area_m2 != payload.new_collector_area_m2 && !(!payload.new_collector_area_m2.is_finite() || payload.new_collector_area_m2 <= 0.0) => vec![vocabulary::change_solar_thermal_system_collector_area(payload.id, item.collector_area_m2)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
