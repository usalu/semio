//! 🔺️ Sparse diff builder for `DeleteAnnualSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteAnnualSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.people.iter().any(|row| row.schedule_id == payload.id || row.activity_schedule_id == payload.id) || base.model.lighting.iter().any(|row| row.schedule_id == payload.id) || base.model.equipment.iter().any(|row| row.schedule_id == payload.id) || base.model.infiltrations.iter().any(|row| row.schedule_id == payload.id) || base.model.mechanical_ventilations.iter().any(|row| row.schedule_id == payload.id) || base.model.thermostats.iter().any(|row| row.heating_setpoint_schedule_id == payload.id || row.cooling_setpoint_schedule_id == payload.id) || base.model.humidistats.iter().any(|row| row.humidifying_setpoint_schedule_id == payload.id || row.dehumidifying_setpoint_schedule_id == payload.id) || base.model.setpoint_managers.iter().any(|row| row.schedule_id == Some(payload.id)) || base.model.shading_surfaces.iter().any(|row| row.transmittance_schedule_id == Some(payload.id)) || base.model.shw_systems.iter().any(|row| row.schedule_id == payload.id) || base.model.refrigeration_systems.iter().any(|row| row.defrost_schedule_id == payload.id) || base.model.water_systems.iter().any(|row| row.schedule_id == payload.id) || base.model.faults.iter().any(|row| row.start_schedule_id == payload.id) || base.model.schedules.weekly.iter().any(|row| row.daily_schedule_ids.contains(&payload.id)) || base.model.schedules.annual.iter().any(|row| row.default_daily_schedule_id == payload.id || row.holiday_daily_schedule_id == Some(payload.id) || row.rules.iter().any(|rule| rule.daily_schedule_id == payload.id)) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule {} is still referenced by the document.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.schedules.annual.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
