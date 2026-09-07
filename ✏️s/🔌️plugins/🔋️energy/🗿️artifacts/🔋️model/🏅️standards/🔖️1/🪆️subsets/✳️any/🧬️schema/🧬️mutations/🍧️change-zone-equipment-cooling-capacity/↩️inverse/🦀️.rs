//! ↩️ Inverse for `ChangeZoneEquipmentCoolingCapacity` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeZoneEquipmentCoolingCapacity, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.zone_equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.cooling_capacity_w != payload.new_cooling_capacity_w && !((!payload.new_cooling_capacity_w.is_finite() || payload.new_cooling_capacity_w < 0.0)) => vec![vocabulary::change_zone_equipment_cooling_capacity(payload.id, item.cooling_capacity_w)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
