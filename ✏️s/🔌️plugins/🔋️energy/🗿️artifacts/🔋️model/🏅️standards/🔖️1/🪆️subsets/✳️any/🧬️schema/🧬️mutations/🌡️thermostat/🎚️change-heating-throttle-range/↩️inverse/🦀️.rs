//! ↩️ Inverse for `ChangeThermostatHeatingThrottleRange` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeThermostatHeatingThrottleRange, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeThermostatHeatingThrottleRange>,
{
    Ok((|| {
    match base.model.thermostats.iter().find(|item| item.id == payload.id) {
        Some(item) if item.heating_throttle_range_k != payload.new_heating_throttle_range_k && !(!payload.new_heating_throttle_range_k.is_finite() || payload.new_heating_throttle_range_k <= 0.0) => {
            vec![vocabulary::change_thermostat_heating_throttle_range(payload.id, item.heating_throttle_range_k)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
