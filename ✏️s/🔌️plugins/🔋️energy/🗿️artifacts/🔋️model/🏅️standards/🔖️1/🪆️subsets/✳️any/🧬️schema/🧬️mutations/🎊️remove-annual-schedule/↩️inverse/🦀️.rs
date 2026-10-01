//! ↩️ Inverse for `RemoveAnnualScheduleHoliday` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveAnnualScheduleHoliday, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(item) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(position) = item.holiday_dates.iter().position(|holiday| *holiday == (payload.year, payload.month, payload.day)) else {
        return Vec::new();
    };
    vec![vocabulary::add_annual_schedule_holiday(payload.id, position as u32, payload.year, payload.month, payload.day)]
}
//#endregion 🔖️Inverse
