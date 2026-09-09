//! ↩️ Inverse for `CreateDailySchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateDailySchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id)
        || payload.index as usize > base.model.schedules.daily.len()
        || payload.hourly_values.len() != 24
        || payload.hourly_values.iter().any(|value| !value.is_finite())
        || payload.limits_min.is_some() != payload.limits_max.is_some()
        || matches!((payload.limits_min, payload.limits_max), (Some(min), Some(max)) if !min.is_finite() || !max.is_finite() || min > max)
    {
        return Vec::new();
    }
    vec![vocabulary::delete_daily_schedule(payload.id)]
}
//#endregion 🔖️Inverse
