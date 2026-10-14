//! ↩️ Inverse for `ReplaceTimeSeriesScheduleValues` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ReplaceTimeSeriesScheduleValues, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ReplaceTimeSeriesScheduleValues>,
{
    Ok((|| {
    match base.model.schedules.time_series.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.values == payload.new_values) && !payload.new_values.is_empty() && payload.new_values.iter().all(|value| value.is_finite()) => vec![vocabulary::replace_time_series_schedule_values(payload.id, item.values.clone())],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
