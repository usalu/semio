//! 🔺️ Sparse diff builder for `CreateHumidistat` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateHumidistat, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.humidistats.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Humidistat {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.humidifying_setpoint_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.humidifying_setpoint_schedule_id.0), [payload.humidifying_setpoint_schedule_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.dehumidifying_setpoint_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.dehumidifying_setpoint_schedule_id.0), [payload.dehumidifying_setpoint_schedule_id.0.to_string()]);
    }
    if !payload.humidifying_throttle_range.is_finite() || payload.humidifying_throttle_range <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A humidifying throttle range must be a positive finite number, got {}.", payload.humidifying_throttle_range), [payload.id.0.to_string()]);
    }
    if !payload.dehumidifying_throttle_range.is_finite() || payload.dehumidifying_throttle_range <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A dehumidifying throttle range must be a positive finite number, got {}.", payload.dehumidifying_throttle_range), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.humidistats.push(crate::model::Humidistat {
        id: payload.id,
        zone_id: payload.zone_id,
        humidifying_setpoint_schedule_id: payload.humidifying_setpoint_schedule_id,
        dehumidifying_setpoint_schedule_id: payload.dehumidifying_setpoint_schedule_id,
        humidifying_throttle_range: payload.humidifying_throttle_range,
        dehumidifying_throttle_range: payload.dehumidifying_throttle_range,
    });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
