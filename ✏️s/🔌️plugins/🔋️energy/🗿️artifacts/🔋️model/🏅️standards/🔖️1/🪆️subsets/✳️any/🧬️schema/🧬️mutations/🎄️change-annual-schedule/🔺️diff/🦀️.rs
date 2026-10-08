//! 🔺️ Sparse diff builder for `ChangeAnnualScheduleHolidayDailySchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, AnnualSchedulePatch, ModelPatch, OptionChange, Rows, ScheduleSetPatch};
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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Annual schedule {} already carries this holiday_daily_schedule_id: {:?}.", payload.id.0, payload.new_holiday_daily_schedule_id));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { annual: Rows::modifying(AnnualSchedulePatch { holiday_daily_schedule_id: OptionChange::assign(payload.new_holiday_daily_schedule_id), ..AnnualSchedulePatch::of(payload.id) }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
