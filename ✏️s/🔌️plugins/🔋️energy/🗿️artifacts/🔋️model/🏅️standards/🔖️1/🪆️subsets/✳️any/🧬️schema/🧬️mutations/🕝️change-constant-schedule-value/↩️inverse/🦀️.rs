//! ↩️ Inverse for `ChangeConstantScheduleValue` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeConstantScheduleValue, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.constants.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.value == payload.new_value) && payload.new_value.is_finite() => vec![vocabulary::change_constant_schedule_value(payload.id, item.value)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
