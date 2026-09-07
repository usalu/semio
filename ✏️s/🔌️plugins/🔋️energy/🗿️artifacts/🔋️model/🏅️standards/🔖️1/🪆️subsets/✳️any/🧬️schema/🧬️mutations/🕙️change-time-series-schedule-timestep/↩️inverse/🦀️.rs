//! ↩️ Inverse for `ChangeTimeSeriesScheduleTimestep` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeTimeSeriesScheduleTimestep, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.time_series.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.timestep_seconds == payload.new_timestep_seconds) && payload.new_timestep_seconds != 0 => vec![vocabulary::change_time_series_schedule_timestep(payload.id, item.timestep_seconds)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
