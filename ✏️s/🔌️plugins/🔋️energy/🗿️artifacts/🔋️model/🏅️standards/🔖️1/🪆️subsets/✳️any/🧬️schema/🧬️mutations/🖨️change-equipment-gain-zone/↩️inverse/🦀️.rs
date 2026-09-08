//! ↩️ Inverse for `ChangeEquipmentGainZone` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeEquipmentGainZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.zone_id != payload.new_zone_id && base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) => vec![vocabulary::change_equipment_gain_zone(payload.id, item.zone_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
