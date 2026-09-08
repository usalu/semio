//! ↩️ Inverse for `ChangeZoneEquipmentPriority` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeZoneEquipmentPriority, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.priority != payload.new_priority && !(payload.new_priority == 0) => vec![vocabulary::change_zone_equipment_priority(payload.id, item.priority)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
