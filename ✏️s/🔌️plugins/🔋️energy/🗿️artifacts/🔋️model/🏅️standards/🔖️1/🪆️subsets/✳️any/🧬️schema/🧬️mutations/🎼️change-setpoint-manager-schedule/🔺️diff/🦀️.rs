//! 🔺️ Sparse diff builder for `ChangeSetpointManagerSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSetpointManagerSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.setpoint_managers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Setpoint manager {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_schedule_present && payload.new_schedule_id.0 != 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "An absent setpoint manager schedule carries the id zero.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.new_schedule_present && (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_schedule_id))) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} is not defined by this model.", payload.new_schedule_id.0), [payload.new_schedule_id.0.to_string()]);
    }
    let value = payload.new_schedule_present.then_some(payload.new_schedule_id);
    if existing.schedule_id == value {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Setpoint manager {} already reads that schedule.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.setpoint_managers.iter_mut().find(|item| item.id == payload.id) {
        item.schedule_id = value;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
