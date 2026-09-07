//! ↩️ Inverse for `ChangePlantLoopType` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangePlantLoopType, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.loop_type != payload.new_loop_type => vec![vocabulary::change_plant_loop_type(payload.id, item.loop_type)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
