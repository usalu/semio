//! ↩️ Inverse for `ChangeRefrigerationSystemDefrostSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeRefrigerationSystemDefrostSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) {
        Some(item)
            if item.defrost_schedule_id != payload.new_defrost_schedule_id
                && (base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_defrost_schedule_id)
                    || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_defrost_schedule_id)
                    || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_defrost_schedule_id)
                    || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_defrost_schedule_id)
                    || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_defrost_schedule_id)) =>
        {
            vec![vocabulary::change_refrigeration_system_defrost_schedule(payload.id, item.defrost_schedule_id)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
