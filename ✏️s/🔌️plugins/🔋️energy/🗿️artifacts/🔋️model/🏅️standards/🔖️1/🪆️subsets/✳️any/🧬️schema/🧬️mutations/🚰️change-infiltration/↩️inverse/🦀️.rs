//! ↩️ Inverse for `ChangeInfiltrationDischargeCoefficient` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeInfiltrationDischargeCoefficient, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.discharge_coefficient != payload.new_discharge_coefficient && !(!payload.new_discharge_coefficient.is_finite() || payload.new_discharge_coefficient <= 0.0) => {
            vec![vocabulary::change_infiltration_discharge_coefficient(payload.id, item.discharge_coefficient)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
