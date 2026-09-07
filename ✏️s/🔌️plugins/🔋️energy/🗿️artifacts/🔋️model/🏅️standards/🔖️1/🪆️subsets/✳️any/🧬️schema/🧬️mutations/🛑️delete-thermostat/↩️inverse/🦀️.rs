//! ↩️ Inverse for `DeleteThermostat` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteThermostat, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.thermostats.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_thermostat(item.id, item.zone_id, item.heating_setpoint_schedule_id, item.cooling_setpoint_schedule_id, item.heating_throttle_range_k, item.cooling_throttle_range_k)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
