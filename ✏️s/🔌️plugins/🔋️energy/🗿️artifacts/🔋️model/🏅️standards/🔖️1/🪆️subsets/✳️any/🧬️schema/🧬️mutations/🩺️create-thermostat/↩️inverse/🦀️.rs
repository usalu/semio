//! ↩️ Inverse for `CreateThermostat` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateThermostat, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.thermostats.iter().any(|item| item.id == payload.id)) || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id)) || (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id))) || (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id))) || (!payload.heating_throttle_range_k.is_finite() || payload.heating_throttle_range_k <= 0.0) || (!payload.cooling_throttle_range_k.is_finite() || payload.cooling_throttle_range_k <= 0.0) {
        return Vec::new();
    }
    vec![vocabulary::delete_thermostat(payload.id)]
}
//#endregion 🔖️Inverse
