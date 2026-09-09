//! ↩️ Inverse for `ChangeThermostatCoolingThrottleRange` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeThermostatCoolingThrottleRange, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.thermostats.iter().find(|item| item.id == payload.id) {
        Some(item) if item.cooling_throttle_range_k != payload.new_cooling_throttle_range_k && !(!payload.new_cooling_throttle_range_k.is_finite() || payload.new_cooling_throttle_range_k <= 0.0) => {
            vec![vocabulary::change_thermostat_cooling_throttle_range(payload.id, item.cooling_throttle_range_k)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
