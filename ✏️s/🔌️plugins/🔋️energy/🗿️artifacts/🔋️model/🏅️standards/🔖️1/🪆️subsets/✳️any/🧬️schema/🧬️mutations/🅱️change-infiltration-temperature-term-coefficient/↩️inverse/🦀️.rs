//! ↩️ Inverse for `ChangeInfiltrationTemperatureTermCoefficient` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeInfiltrationTemperatureTermCoefficient, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.temperature_term_coefficient != payload.new_temperature_term_coefficient && !(!payload.new_temperature_term_coefficient.is_finite() || payload.new_temperature_term_coefficient < 0.0) => vec![vocabulary::change_infiltration_temperature_term_coefficient(payload.id, item.temperature_term_coefficient)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
