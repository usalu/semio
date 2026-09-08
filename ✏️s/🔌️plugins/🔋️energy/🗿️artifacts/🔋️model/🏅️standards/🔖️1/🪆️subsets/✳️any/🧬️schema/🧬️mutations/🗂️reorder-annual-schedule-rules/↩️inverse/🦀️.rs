//! ↩️ Inverse for `ReorderAnnualScheduleRules` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ReorderAnnualScheduleRules, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.annual.iter().find(|item| item.id == payload.id) {
        Some(item) if payload.from != payload.to && (payload.from as usize) < item.rules.len() && (payload.to as usize) < item.rules.len() => vec![vocabulary::reorder_annual_schedule_rules(payload.id, payload.to, payload.from)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
