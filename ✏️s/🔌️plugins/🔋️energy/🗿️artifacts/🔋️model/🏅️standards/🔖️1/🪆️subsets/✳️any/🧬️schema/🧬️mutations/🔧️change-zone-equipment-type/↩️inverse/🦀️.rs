//! ↩️ Inverse for `ChangeZoneEquipmentType` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeZoneEquipmentType, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.equipment_type != payload.new_equipment_type => vec![vocabulary::change_zone_equipment_type(payload.id, item.equipment_type.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
