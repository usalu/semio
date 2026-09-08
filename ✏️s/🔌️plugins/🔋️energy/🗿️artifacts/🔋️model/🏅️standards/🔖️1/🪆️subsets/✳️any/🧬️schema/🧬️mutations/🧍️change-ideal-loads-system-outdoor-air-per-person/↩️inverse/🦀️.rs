//! ↩️ Inverse for `ChangeIdealLoadsSystemOutdoorAirPerPerson` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeIdealLoadsSystemOutdoorAirPerPerson, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.ideal_loads.iter().find(|item| item.id == payload.id) {
        Some(item) if item.outdoor_air_per_person_m3_s != payload.new_outdoor_air_per_person_m3_s && !(!payload.new_outdoor_air_per_person_m3_s.is_finite() || payload.new_outdoor_air_per_person_m3_s < 0.0) => vec![vocabulary::change_ideal_loads_system_outdoor_air_per_person(payload.id, item.outdoor_air_per_person_m3_s)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
