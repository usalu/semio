//! ↩️ Inverse for `ReplaceDailyScheduleHourlyValues` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ReplaceDailyScheduleHourlyValues, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.daily.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.hourly_values.as_slice() == payload.new_hourly_values.as_slice()) && payload.new_hourly_values.len() == 24 && payload.new_hourly_values.iter().all(|value| value.is_finite()) => {
            vec![vocabulary::replace_daily_schedule_hourly_values(payload.id, item.hourly_values.to_vec())]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
