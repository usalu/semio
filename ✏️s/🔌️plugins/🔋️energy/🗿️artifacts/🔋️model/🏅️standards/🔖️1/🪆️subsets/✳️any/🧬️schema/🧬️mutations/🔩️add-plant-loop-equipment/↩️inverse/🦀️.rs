//! ↩️ Inverse for `AddPlantLoopEquipment` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddPlantLoopEquipment, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(existing) if !((payload.equipment_id.0 == 0) || (existing.equipment_ids.contains(&payload.equipment_id))) => vec![vocabulary::remove_plant_loop_equipment(payload.id, payload.equipment_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
