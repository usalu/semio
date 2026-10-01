//! ↩️ Inverse for `InsertAnnualScheduleRule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::InsertAnnualScheduleRule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
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
}
//#endregion 🔖️Inverse
