//! ↩️ Inverse for `ReplaceDailyScheduleHourlyValues` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ReplaceDailyScheduleHourlyValues, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ReplaceDailyScheduleHourlyValues>,
{
    Ok((|| {
    match base.model.schedules.daily.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.hourly_values.as_slice() == payload.new_hourly_values.as_slice()) && payload.new_hourly_values.iter().all(|value| value.is_finite()) => {
            vec![vocabulary::replace_daily_schedule_hourly_values(payload.id, item.hourly_values)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
