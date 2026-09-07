//! ↩️ Inverse for `ChangeWaterSystemSchedule` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeWaterSystemSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.water_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.schedule_id != payload.new_schedule_id && (base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_schedule_id)) => vec![vocabulary::change_water_system_schedule(payload.id, item.schedule_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
