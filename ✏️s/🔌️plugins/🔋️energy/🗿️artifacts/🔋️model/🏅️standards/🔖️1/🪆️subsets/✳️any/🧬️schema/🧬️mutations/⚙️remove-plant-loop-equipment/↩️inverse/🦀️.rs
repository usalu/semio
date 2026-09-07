//! ↩️ Inverse for `RemovePlantLoopEquipment` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemovePlantLoopEquipment, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(existing) if !(!existing.equipment_ids.contains(&payload.equipment_id)) => vec![vocabulary::add_plant_loop_equipment(payload.id, payload.equipment_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
