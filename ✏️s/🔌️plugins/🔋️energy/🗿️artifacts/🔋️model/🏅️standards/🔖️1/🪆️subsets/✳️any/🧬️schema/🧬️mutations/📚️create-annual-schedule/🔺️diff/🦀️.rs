//! 🔺️ Sparse diff builder for `CreateAnnualSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateAnnualSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Schedule {} is already defined.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.schedules.annual.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} annual schedules.", payload.index, base.model.schedules.annual.len()), [payload.id.0.to_string()]);
    }
    if !base.model.schedules.daily.iter().any(|row| row.id == payload.default_daily_schedule_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", payload.default_daily_schedule_id.0), [payload.default_daily_schedule_id.0.to_string()]);
    }
    if let Some(holiday) = payload.holiday_daily_schedule_id {
        if !base.model.schedules.daily.iter().any(|row| row.id == holiday) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", holiday.0), [holiday.0.to_string()]);
        }
    }
    let mut model = base.model.clone();
    model.schedules.annual.insert(payload.index as usize, crate::schedule::AnnualSchedule { id: payload.id, rules: Vec::new(), default_daily_schedule_id: payload.default_daily_schedule_id, holiday_daily_schedule_id: payload.holiday_daily_schedule_id, holiday_dates: Vec::new() });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
