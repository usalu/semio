//! ↩️ Inverse for `RemoveOutputVariable` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RemoveOutputVariable, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RemoveOutputVariable> + From<crate::AddOutputVariable>,
{
    Ok((|| {
    match base.model.output_variables.iter().enumerate().find(|(_, spec)| spec.name == payload.name && spec.key == payload.key) {
        Some((index, spec)) => vec![vocabulary::add_output_variable(spec.name.clone(), spec.key.clone(), spec.reporting_frequency, Some(index as u32))],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
