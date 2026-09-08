//! ↩️ Inverse for `RemoveOutputVariable` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveOutputVariable, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.output_variables.iter().find(|spec| spec.name == payload.name && spec.key == payload.key) {
        Some(spec) => vec![vocabulary::add_output_variable(spec.name.clone(), spec.key.clone(), spec.reporting_frequency)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
