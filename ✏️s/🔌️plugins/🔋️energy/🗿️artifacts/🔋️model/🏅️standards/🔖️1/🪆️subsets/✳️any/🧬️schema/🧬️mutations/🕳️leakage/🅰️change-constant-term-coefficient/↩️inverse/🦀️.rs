//! ↩️ Inverse for `ChangeInfiltrationConstantTermCoefficient` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeInfiltrationConstantTermCoefficient, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeInfiltrationConstantTermCoefficient>,
{
    Ok((|| {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.constant_term_coefficient != payload.new_constant_term_coefficient && !(!payload.new_constant_term_coefficient.is_finite() || payload.new_constant_term_coefficient < 0.0) => {
            vec![vocabulary::change_infiltration_constant_term_coefficient(payload.id, item.constant_term_coefficient)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
