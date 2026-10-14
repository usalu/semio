//! ↩️ Inverse for `RemoveAnnualScheduleRule` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RemoveAnnualScheduleRule, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RemoveAnnualScheduleRule> + From<crate::InsertAnnualScheduleRule>,
{
    Ok((|| {
    let Some(item) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(rule) = item.rules.get(payload.index as usize) else {
        return Vec::new();
    };
    vec![vocabulary::insert_annual_schedule_rule(payload.id, payload.index, rule.start_month, rule.start_day, rule.end_month, rule.end_day, rule.daily_schedule_id)]

    })())
}
//#endregion 🔖️Inverse
