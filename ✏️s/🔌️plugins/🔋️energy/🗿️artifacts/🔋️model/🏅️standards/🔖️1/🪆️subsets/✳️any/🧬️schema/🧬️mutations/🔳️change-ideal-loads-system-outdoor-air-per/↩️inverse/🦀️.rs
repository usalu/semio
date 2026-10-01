//! ↩️ Inverse for `ChangeIdealLoadsSystemOutdoorAirPerArea` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeIdealLoadsSystemOutdoorAirPerArea, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.ideal_loads.iter().find(|item| item.id == payload.id) {
        Some(item) if item.outdoor_air_per_area_m3_s_m2 != payload.new_outdoor_air_per_area_m3_s_m2 && !(!payload.new_outdoor_air_per_area_m3_s_m2.is_finite() || payload.new_outdoor_air_per_area_m3_s_m2 < 0.0) => {
            vec![vocabulary::change_ideal_loads_system_outdoor_air_per_area(payload.id, item.outdoor_air_per_area_m3_s_m2)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
