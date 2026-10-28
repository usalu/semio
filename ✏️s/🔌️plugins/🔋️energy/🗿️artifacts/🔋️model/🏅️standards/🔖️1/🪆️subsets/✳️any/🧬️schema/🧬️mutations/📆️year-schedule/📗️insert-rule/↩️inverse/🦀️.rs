//! ↩️ Inverse for `InsertAnnualScheduleRule` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::InsertAnnualScheduleRule, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::InsertAnnualScheduleRule> + From<crate::RemoveAnnualScheduleRule>,
{
    Ok((|| {
    let Some(item) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.index as usize > item.rules.len()
        || !(1..=12).contains(&payload.start_month)
        || !(1..=12).contains(&payload.end_month)
        || !(1..=31).contains(&payload.start_day)
        || !(1..=31).contains(&payload.end_day)
        || !base.model.schedules.daily.iter().any(|row| row.id == payload.daily_schedule_id)
    {
        return Vec::new();
    }
    vec![vocabulary::remove_annual_schedule_rule(payload.id, payload.index)]

    })())
}
//#endregion 🔖️Inverse
