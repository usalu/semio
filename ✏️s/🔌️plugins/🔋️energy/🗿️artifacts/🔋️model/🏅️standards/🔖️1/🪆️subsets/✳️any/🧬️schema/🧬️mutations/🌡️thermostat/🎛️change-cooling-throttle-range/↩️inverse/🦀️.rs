//! ↩️ Inverse for `ChangeThermostatCoolingThrottleRange` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeThermostatCoolingThrottleRange, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeThermostatCoolingThrottleRange>,
{
    Ok((|| {
    match base.model.thermostats.iter().find(|item| item.id == payload.id) {
        Some(item) if item.cooling_throttle_range_k != payload.new_cooling_throttle_range_k && !(!payload.new_cooling_throttle_range_k.is_finite() || payload.new_cooling_throttle_range_k <= 0.0) => {
            vec![vocabulary::change_thermostat_cooling_throttle_range(payload.id, item.cooling_throttle_range_k)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
