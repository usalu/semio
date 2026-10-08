//! ↩️ Inverse for `RemoveOutputVariable` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveOutputVariable, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.model.output_variables.iter().enumerate().find(|(_, spec)| spec.name == payload.name && spec.key == payload.key) {
        Some((index, spec)) => vec![vocabulary::add_output_variable(spec.name.clone(), spec.key.clone(), spec.reporting_frequency, Some(index as u32))],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
