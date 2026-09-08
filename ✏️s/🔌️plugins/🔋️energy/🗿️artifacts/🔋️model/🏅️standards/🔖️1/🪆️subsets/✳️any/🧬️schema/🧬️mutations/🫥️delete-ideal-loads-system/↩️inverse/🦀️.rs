//! ↩️ Inverse for `DeleteIdealLoadsSystem` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteIdealLoadsSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.ideal_loads.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_ideal_loads_system(item.id, item.zone_id, item.max_heating_supply_air_temp_c, item.min_cooling_supply_air_temp_c, item.max_heating_capacity_w.is_some(), item.max_heating_capacity_w.unwrap_or(0.0), item.max_cooling_capacity_w.is_some(), item.max_cooling_capacity_w.unwrap_or(0.0), item.outdoor_air_per_person_m3_s, item.outdoor_air_per_area_m3_s_m2)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
