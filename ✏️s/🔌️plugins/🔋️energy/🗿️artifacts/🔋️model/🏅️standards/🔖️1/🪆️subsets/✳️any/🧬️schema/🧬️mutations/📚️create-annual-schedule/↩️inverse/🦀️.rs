//! ↩️ Inverse for `CreateAnnualSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateAnnualSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id)
        || payload.index as usize > base.model.schedules.annual.len()
        || !base.model.schedules.daily.iter().any(|row| row.id == payload.default_daily_schedule_id)
        || payload.holiday_daily_schedule_id.is_some_and(|holiday| !base.model.schedules.daily.iter().any(|row| row.id == holiday))
    {
        return Vec::new();
    }
    vec![vocabulary::delete_annual_schedule(payload.id)]
}
//#endregion 🔖️Inverse
