//! ↩️ Inverse for `ChangeInfiltrationVelocitySquaredTermCoefficient` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeInfiltrationVelocitySquaredTermCoefficient, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.velocity_squared_term_coefficient != payload.new_velocity_squared_term_coefficient && !(!payload.new_velocity_squared_term_coefficient.is_finite() || payload.new_velocity_squared_term_coefficient < 0.0) => {
            vec![vocabulary::change_infiltration_velocity_squared_term_coefficient(payload.id, item.velocity_squared_term_coefficient)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
