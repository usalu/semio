//! ↩️ Inverse for `ChangeWeeklyScheduleDay` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeWeeklyScheduleDay, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.day_index > 6 || !base.model.schedules.daily.iter().any(|row| row.id == payload.new_daily_schedule_id) {
        return Vec::new();
    }
    match base.model.schedules.weekly.iter().find(|item| item.id == payload.id) {
        Some(item) if item.daily_schedule_ids[payload.day_index as usize] != payload.new_daily_schedule_id => vec![vocabulary::change_weekly_schedule_day(payload.id, payload.day_index, item.daily_schedule_ids[payload.day_index as usize])],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
