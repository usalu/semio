//! ↩️ Inverse for `ChangeInfiltrationVelocityTermCoefficient` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeInfiltrationVelocityTermCoefficient, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeInfiltrationVelocityTermCoefficient>,
{
    Ok((|| {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.velocity_term_coefficient != payload.new_velocity_term_coefficient && !(!payload.new_velocity_term_coefficient.is_finite() || payload.new_velocity_term_coefficient < 0.0) => {
            vec![vocabulary::change_infiltration_velocity_term_coefficient(payload.id, item.velocity_term_coefficient)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
