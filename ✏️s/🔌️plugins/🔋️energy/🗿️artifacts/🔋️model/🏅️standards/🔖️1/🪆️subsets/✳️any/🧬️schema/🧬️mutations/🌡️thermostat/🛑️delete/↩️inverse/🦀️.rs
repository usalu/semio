//! ↩️ Inverse for `DeleteThermostat` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteThermostat, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.model.thermostats.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((index, item)) => vec![vocabulary::create_thermostat(item.id, item.zone_id, item.heating_setpoint_schedule_id, item.cooling_setpoint_schedule_id, item.heating_throttle_range_k, item.cooling_throttle_range_k, Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
