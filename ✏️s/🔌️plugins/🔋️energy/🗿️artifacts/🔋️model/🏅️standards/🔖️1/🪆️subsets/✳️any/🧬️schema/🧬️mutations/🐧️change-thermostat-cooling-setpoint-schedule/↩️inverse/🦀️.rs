//! ↩️ Inverse for `ChangeThermostatCoolingSetpointSchedule` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeThermostatCoolingSetpointSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.thermostats.iter().find(|item| item.id == payload.id) {
        Some(item) if item.cooling_setpoint_schedule_id != payload.new_cooling_setpoint_schedule_id && !((!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id)))) => vec![vocabulary::change_thermostat_cooling_setpoint_schedule(payload.id, item.cooling_setpoint_schedule_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
