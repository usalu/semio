//! ↩️ Inverse for `AddOutputVariable` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddOutputVariable, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.name.trim().is_empty() || base.model.output_variables.iter().any(|spec| spec.name == payload.name && spec.key == payload.key) {
        return Vec::new();
    }
    vec![vocabulary::remove_output_variable(payload.name.clone(), payload.key.clone())]
}
//#endregion 🔖️Inverse
