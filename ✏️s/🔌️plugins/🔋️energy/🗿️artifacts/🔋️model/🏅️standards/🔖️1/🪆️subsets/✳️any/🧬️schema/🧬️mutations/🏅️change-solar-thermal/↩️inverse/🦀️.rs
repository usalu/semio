//! ↩️ Inverse for `ChangeSolarThermalSystemEfficiency` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSolarThermalSystemEfficiency, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.efficiency != payload.new_efficiency && !(!(payload.new_efficiency > 0.0 && payload.new_efficiency <= 1.0)) => vec![vocabulary::change_solar_thermal_system_efficiency(payload.id, item.efficiency)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
