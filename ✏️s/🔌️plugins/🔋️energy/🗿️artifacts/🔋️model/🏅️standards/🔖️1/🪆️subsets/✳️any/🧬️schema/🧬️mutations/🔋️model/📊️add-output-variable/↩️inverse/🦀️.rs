//! ↩️ Inverse for `AddOutputVariable` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::AddOutputVariable, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::AddOutputVariable> + From<crate::RemoveOutputVariable>,
{
    Ok((|| {
    if payload.name.trim().is_empty() || base.model.output_variables.iter().any(|spec| spec.name == payload.name && spec.key == payload.key) {
        return Vec::new();
    }
    if payload.index.is_some_and(|index| index as usize > base.model.output_variables.len()) {
        return Vec::new();
    }
    vec![vocabulary::remove_output_variable(payload.name.clone(), payload.key.clone())]

    })())
}
//#endregion 🔖️Inverse
