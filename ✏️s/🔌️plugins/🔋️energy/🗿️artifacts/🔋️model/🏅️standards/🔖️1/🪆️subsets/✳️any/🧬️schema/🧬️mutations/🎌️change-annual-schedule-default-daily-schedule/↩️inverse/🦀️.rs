//! ↩️ Inverse for `ChangeAnnualScheduleDefaultDailySchedule` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeAnnualScheduleDefaultDailySchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.annual.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.default_daily_schedule_id == payload.new_default_daily_schedule_id) && base.model.schedules.daily.iter().any(|row| row.id == payload.new_default_daily_schedule_id) => vec![vocabulary::change_annual_schedule_default_daily_schedule(payload.id, item.default_daily_schedule_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
