//! ↩️ Inverse for `CreateIdealLoadsSystem` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateIdealLoadsSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.ideal_loads.iter().any(|item| item.id == payload.id)) || (!base.model.zones.iter().any(|zone| zone.id == payload.zone_id)) || (!payload.max_heating_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.max_heating_supply_air_temp_c)) || (!payload.min_cooling_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.min_cooling_supply_air_temp_c)) || (!payload.outdoor_air_per_person_m3_s.is_finite() || payload.outdoor_air_per_person_m3_s < 0.0) || (!payload.outdoor_air_per_area_m3_s_m2.is_finite() || payload.outdoor_air_per_area_m3_s_m2 < 0.0) || (!payload.max_heating_capacity_present && payload.max_heating_capacity_w != 0.0) || (!payload.max_cooling_capacity_present && payload.max_cooling_capacity_w != 0.0) || (payload.max_heating_capacity_present && (!payload.max_heating_capacity_w.is_finite() || payload.max_heating_capacity_w <= 0.0)) || (payload.max_cooling_capacity_present && (!payload.max_cooling_capacity_w.is_finite() || payload.max_cooling_capacity_w <= 0.0)) {
        return Vec::new();
    }
    vec![vocabulary::delete_ideal_loads_system(payload.id)]
}
//#endregion 🔖️Inverse
