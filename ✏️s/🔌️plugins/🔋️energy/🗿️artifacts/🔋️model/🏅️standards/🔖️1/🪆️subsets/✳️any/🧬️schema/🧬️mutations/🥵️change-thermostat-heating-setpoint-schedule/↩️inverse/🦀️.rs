//! ↩️ Inverse for `ChangeThermostatHeatingSetpointSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeThermostatHeatingSetpointSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.thermostats.iter().find(|item| item.id == payload.id) {
        Some(item)
            if item.heating_setpoint_schedule_id != payload.new_heating_setpoint_schedule_id
                && !(!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
                    || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
                    || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
                    || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
                    || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id))) =>
        {
            vec![vocabulary::change_thermostat_heating_setpoint_schedule(payload.id, item.heating_setpoint_schedule_id)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
