//! ↩️ Inverse for `ChangeZoneEquipmentPriority` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeZoneEquipmentPriority, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.priority != payload.new_priority && !(payload.new_priority == 0) => vec![vocabulary::change_zone_equipment_priority(payload.id, item.priority)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
