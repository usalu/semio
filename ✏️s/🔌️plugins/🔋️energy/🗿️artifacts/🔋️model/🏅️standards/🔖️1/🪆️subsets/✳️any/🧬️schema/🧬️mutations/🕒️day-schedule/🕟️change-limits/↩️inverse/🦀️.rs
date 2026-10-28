//! ↩️ Inverse for `ChangeDailyScheduleLimits` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeDailyScheduleLimits, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeDailyScheduleLimits>,
{
    Ok((|| {
    let limits = match (payload.new_limits_min, payload.new_limits_max) {
        (Some(min), Some(max)) => Some(crate::schedule::ScheduleLimits { min, max }),
        _ => None,
    };
    if payload.new_limits_min.is_some() != payload.new_limits_max.is_some() || matches!((payload.new_limits_min, payload.new_limits_max), (Some(min), Some(max)) if !min.is_finite() || !max.is_finite() || min > max) {
        return Vec::new();
    }
    match base.model.schedules.daily.iter().find(|item| item.id == payload.id) {
        Some(item) if item.limits != limits => vec![vocabulary::change_daily_schedule_limits(payload.id, item.limits.map(|old| old.min), item.limits.map(|old| old.max))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
