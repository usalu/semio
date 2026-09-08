//! ↩️ Inverse for `CreateTimeSeriesSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateTimeSeriesSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id) || payload.index as usize > base.model.schedules.time_series.len() || payload.values.is_empty() || payload.values.iter().any(|value| !value.is_finite()) || payload.timestep_seconds == 0 {
        return Vec::new();
    }
    vec![vocabulary::delete_time_series_schedule(payload.id)]
}
//#endregion 🔖️Inverse
