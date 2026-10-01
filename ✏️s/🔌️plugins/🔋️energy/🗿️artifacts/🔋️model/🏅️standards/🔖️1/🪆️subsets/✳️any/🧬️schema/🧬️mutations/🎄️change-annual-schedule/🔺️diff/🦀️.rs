//! 🔺️ Sparse diff builder for `ChangeAnnualScheduleHolidayDailySchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeAnnualScheduleHolidayDailySchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if let Some(holiday) = payload.new_holiday_daily_schedule_id {
        if !base.model.schedules.daily.iter().any(|row| row.id == holiday) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Daily schedule {} does not exist.", holiday.0), [holiday.0.to_string()]);
        }
    }
    if existing.holiday_daily_schedule_id == payload.new_holiday_daily_schedule_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Annual schedule {} already carries this holiday_daily_schedule_id: {:?}.", payload.id.0, payload.new_holiday_daily_schedule_id));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.holiday_daily_schedule_id = payload.new_holiday_daily_schedule_id;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
