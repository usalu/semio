//! ↩️ Inverse for `CreateWeeklySchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateWeeklySchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id) || payload.index as usize > base.model.schedules.weekly.len() || payload.daily_schedule_ids.len() != 7 || payload.daily_schedule_ids.iter().any(|candidate| !base.model.schedules.daily.iter().any(|row| row.id == *candidate)) {
        return Vec::new();
    }
    vec![vocabulary::delete_weekly_schedule(payload.id)]
}
//#endregion 🔖️Inverse
