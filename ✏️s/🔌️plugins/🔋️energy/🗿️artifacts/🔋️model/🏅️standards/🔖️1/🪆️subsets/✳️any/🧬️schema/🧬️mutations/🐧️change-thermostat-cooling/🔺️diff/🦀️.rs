//! 🔺️ Sparse diff builder for `ChangeThermostatCoolingSetpointSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ThermostatPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeThermostatCoolingSetpointSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermostats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermostat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_cooling_setpoint_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.new_cooling_setpoint_schedule_id.0), [payload.new_cooling_setpoint_schedule_id.0.to_string()]);
    }
    if existing.cooling_setpoint_schedule_id == payload.new_cooling_setpoint_schedule_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Thermostat {} already has that cooling setpoint schedule.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { thermostats: Rows::modifying(ThermostatPatch { cooling_setpoint_schedule_id: Some(payload.new_cooling_setpoint_schedule_id), ..ThermostatPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
