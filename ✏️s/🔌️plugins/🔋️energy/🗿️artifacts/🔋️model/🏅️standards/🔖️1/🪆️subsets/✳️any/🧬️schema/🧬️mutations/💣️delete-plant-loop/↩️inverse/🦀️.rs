//! ↩️ Inverse for `DeletePlantLoop` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeletePlantLoop, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_plant_loop(item.id, item.name.clone(), item.loop_type, item.supply_temperature_c, item.return_temperature_c, item.design_flow_kg_s, item.equipment_ids.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
