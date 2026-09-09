//! ↩️ Inverse for `ChangeFaultStartSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFaultStartSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.faults.iter().find(|item| item.id == payload.id) {
        Some(item)
            if item.start_schedule_id != payload.new_start_schedule_id
                && (base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
                    || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
                    || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
                    || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
                    || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)) =>
        {
            vec![vocabulary::change_fault_start_schedule(payload.id, item.start_schedule_id)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
