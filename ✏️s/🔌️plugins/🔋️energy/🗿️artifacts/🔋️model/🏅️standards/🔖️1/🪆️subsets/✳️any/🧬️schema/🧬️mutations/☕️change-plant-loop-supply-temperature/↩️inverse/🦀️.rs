//! ↩️ Inverse for `ChangePlantLoopSupplyTemperature` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangePlantLoopSupplyTemperature, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.supply_temperature_c != payload.new_supply_temperature_c && !((!payload.new_supply_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.new_supply_temperature_c))) => vec![vocabulary::change_plant_loop_supply_temperature(payload.id, item.supply_temperature_c)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
