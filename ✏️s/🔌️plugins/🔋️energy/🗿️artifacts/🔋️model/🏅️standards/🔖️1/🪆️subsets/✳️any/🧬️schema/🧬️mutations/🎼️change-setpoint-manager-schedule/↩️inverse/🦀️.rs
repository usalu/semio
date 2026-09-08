//! ↩️ Inverse for `ChangeSetpointManagerSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSetpointManagerSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let value = payload.new_schedule_present.then_some(payload.new_schedule_id);
    match base.model.setpoint_managers.iter().find(|item| item.id == payload.id) {
        Some(item) if item.schedule_id != value && !((!payload.new_schedule_present && payload.new_schedule_id.0 != 0) || (payload.new_schedule_present && (!(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_schedule_id))))) => vec![vocabulary::change_setpoint_manager_schedule(payload.id, item.schedule_id.is_some(), item.schedule_id.unwrap_or(crate::model::ScheduleId(0)))],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
