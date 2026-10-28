//! ↩️ Inverse for `ChangeAnnualScheduleHolidayDailySchedule` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeAnnualScheduleHolidayDailySchedule, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeAnnualScheduleHolidayDailySchedule>,
{
    Ok((|| {
    match base.model.schedules.annual.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.holiday_daily_schedule_id == payload.new_holiday_daily_schedule_id) && !payload.new_holiday_daily_schedule_id.is_some_and(|holiday| !base.model.schedules.daily.iter().any(|row| row.id == holiday)) => {
            vec![vocabulary::change_annual_schedule_holiday_daily_schedule(payload.id, item.holiday_daily_schedule_id)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
