//! ↩️ Inverse for `ChangeDailyScheduleInterpolation` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeDailyScheduleInterpolation, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.schedules.daily.iter().find(|item| item.id == payload.id) {
        Some(item) if !(item.interpolation == payload.new_interpolation) => vec![vocabulary::change_daily_schedule_interpolation(payload.id, item.interpolation)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
