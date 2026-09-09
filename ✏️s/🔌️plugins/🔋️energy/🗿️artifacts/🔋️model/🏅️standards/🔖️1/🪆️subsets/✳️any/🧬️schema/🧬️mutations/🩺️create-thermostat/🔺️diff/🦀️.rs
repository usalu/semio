//! 🔺️ Sparse diff builder for `CreateThermostat` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateThermostat, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.thermostats.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Thermostat {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.heating_setpoint_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.heating_setpoint_schedule_id.0), [payload.heating_setpoint_schedule_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.cooling_setpoint_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.cooling_setpoint_schedule_id.0), [payload.cooling_setpoint_schedule_id.0.to_string()]);
    }
    if !payload.heating_throttle_range_k.is_finite() || payload.heating_throttle_range_k <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A heating throttle range must be a positive finite number, got {}.", payload.heating_throttle_range_k), [payload.id.0.to_string()]);
    }
    if !payload.cooling_throttle_range_k.is_finite() || payload.cooling_throttle_range_k <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A cooling throttle range must be a positive finite number, got {}.", payload.cooling_throttle_range_k), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.thermostats.push(crate::model::Thermostat {
        id: payload.id,
        zone_id: payload.zone_id,
        heating_setpoint_schedule_id: payload.heating_setpoint_schedule_id,
        cooling_setpoint_schedule_id: payload.cooling_setpoint_schedule_id,
        heating_throttle_range_k: payload.heating_throttle_range_k,
        cooling_throttle_range_k: payload.cooling_throttle_range_k,
    });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
