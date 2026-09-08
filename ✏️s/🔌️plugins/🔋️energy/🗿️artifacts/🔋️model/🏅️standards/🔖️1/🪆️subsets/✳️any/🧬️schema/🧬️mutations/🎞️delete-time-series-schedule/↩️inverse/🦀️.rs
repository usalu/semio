//! ↩️ Inverse for `DeleteTimeSeriesSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteTimeSeriesSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.schedules.time_series.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.people.iter().any(|row| row.schedule_id == payload.id || row.activity_schedule_id == payload.id) || base.model.lighting.iter().any(|row| row.schedule_id == payload.id) || base.model.equipment.iter().any(|row| row.schedule_id == payload.id) || base.model.infiltrations.iter().any(|row| row.schedule_id == payload.id) || base.model.mechanical_ventilations.iter().any(|row| row.schedule_id == payload.id) || base.model.thermostats.iter().any(|row| row.heating_setpoint_schedule_id == payload.id || row.cooling_setpoint_schedule_id == payload.id) || base.model.humidistats.iter().any(|row| row.humidifying_setpoint_schedule_id == payload.id || row.dehumidifying_setpoint_schedule_id == payload.id) || base.model.setpoint_managers.iter().any(|row| row.schedule_id == Some(payload.id)) || base.model.shading_surfaces.iter().any(|row| row.transmittance_schedule_id == Some(payload.id)) || base.model.shw_systems.iter().any(|row| row.schedule_id == payload.id) || base.model.refrigeration_systems.iter().any(|row| row.defrost_schedule_id == payload.id) || base.model.water_systems.iter().any(|row| row.schedule_id == payload.id) || base.model.faults.iter().any(|row| row.start_schedule_id == payload.id) || base.model.schedules.weekly.iter().any(|row| row.daily_schedule_ids.contains(&payload.id)) || base.model.schedules.annual.iter().any(|row| row.default_daily_schedule_id == payload.id || row.holiday_daily_schedule_id == Some(payload.id) || row.rules.iter().any(|rule| rule.daily_schedule_id == payload.id)) {
        return Vec::new();
    }
    let existing = &base.model.schedules.time_series[index];
    vec![vocabulary::create_time_series_schedule(index as u32, existing.id, existing.values.clone(), existing.timestep_seconds)]
}
//#endregion 🔖️Inverse
