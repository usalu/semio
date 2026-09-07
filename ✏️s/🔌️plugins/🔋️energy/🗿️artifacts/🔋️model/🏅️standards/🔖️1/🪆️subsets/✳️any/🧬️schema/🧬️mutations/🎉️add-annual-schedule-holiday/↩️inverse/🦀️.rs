//! ↩️ Inverse for `AddAnnualScheduleHoliday` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddAnnualScheduleHoliday, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.annual.iter().find(|item| item.id == payload.id) {
        Some(item) if (1..=12).contains(&payload.month) && (1..=31).contains(&payload.day) && payload.index as usize <= item.holiday_dates.len() && !item.holiday_dates.contains(&(payload.year, payload.month, payload.day)) => vec![vocabulary::remove_annual_schedule_holiday(payload.id, payload.year, payload.month, payload.day)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
