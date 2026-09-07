//! ↩️ Inverse for `ChangeFaultTargetEquipment` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFaultTargetEquipment, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.faults.iter().find(|item| item.id == payload.id) {
        Some(item) if item.target_equipment_id != payload.new_target_equipment_id && base.model.ideal_loads.iter().any(|row| row.id == payload.new_target_equipment_id) => vec![vocabulary::change_fault_target_equipment(payload.id, item.target_equipment_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
