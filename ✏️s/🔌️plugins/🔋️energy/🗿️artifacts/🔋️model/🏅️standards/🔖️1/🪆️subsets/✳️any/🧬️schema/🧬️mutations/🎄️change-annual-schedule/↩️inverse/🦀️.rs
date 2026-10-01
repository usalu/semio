//! ↩️ Inverse for `ChangeAnnualScheduleHolidayDailySchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeAnnualScheduleHolidayDailySchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.annual.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.holiday_daily_schedule_id == payload.new_holiday_daily_schedule_id) && !payload.new_holiday_daily_schedule_id.is_some_and(|holiday| !base.model.schedules.daily.iter().any(|row| row.id == holiday)) => {
            vec![vocabulary::change_annual_schedule_holiday_daily_schedule(payload.id, item.holiday_daily_schedule_id)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
