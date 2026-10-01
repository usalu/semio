//! 🔺️ Sparse diff builder for `ChangeThermostatHeatingSetpointSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeThermostatHeatingSetpointSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermostats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermostat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_heating_setpoint_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.new_heating_setpoint_schedule_id.0), [payload.new_heating_setpoint_schedule_id.0.to_string()]);
    }
    if existing.heating_setpoint_schedule_id == payload.new_heating_setpoint_schedule_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Thermostat {} already has that heating setpoint schedule.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.thermostats.iter_mut().find(|item| item.id == payload.id) {
        item.heating_setpoint_schedule_id = payload.new_heating_setpoint_schedule_id;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
