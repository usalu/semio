//! ↩️ Inverse for `ChangePlantLoopReturnTemperature` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangePlantLoopReturnTemperature, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.return_temperature_c != payload.new_return_temperature_c && !(!payload.new_return_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.new_return_temperature_c)) => vec![vocabulary::change_plant_loop_return_temperature(payload.id, item.return_temperature_c)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
