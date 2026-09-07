//! ↩️ Inverse for `CreateEquipmentGain` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateEquipmentGain, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.equipment.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.equipment.len() || !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) || !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id)) {
        return Vec::new();
    }
    vec![vocabulary::delete_equipment_gain(payload.id)]
}
//#endregion 🔖️Inverse
