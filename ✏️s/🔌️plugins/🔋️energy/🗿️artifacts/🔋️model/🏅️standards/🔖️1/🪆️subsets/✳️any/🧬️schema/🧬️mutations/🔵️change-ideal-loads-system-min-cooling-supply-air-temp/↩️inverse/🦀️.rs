//! ↩️ Inverse for `ChangeIdealLoadsSystemMinCoolingSupplyAirTemp` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeIdealLoadsSystemMinCoolingSupplyAirTemp, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.ideal_loads.iter().find(|item| item.id == payload.id) {
        Some(item) if item.min_cooling_supply_air_temp_c != payload.new_min_cooling_supply_air_temp_c && !(!payload.new_min_cooling_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.new_min_cooling_supply_air_temp_c)) => vec![vocabulary::change_ideal_loads_system_min_cooling_supply_air_temp(payload.id, item.min_cooling_supply_air_temp_c)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
