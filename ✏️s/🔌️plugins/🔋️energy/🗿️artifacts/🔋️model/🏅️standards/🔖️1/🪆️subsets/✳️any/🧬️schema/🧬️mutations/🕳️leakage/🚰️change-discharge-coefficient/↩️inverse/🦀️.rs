//! ↩️ Inverse for `ChangeInfiltrationDischargeCoefficient` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeInfiltrationDischargeCoefficient, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeInfiltrationDischargeCoefficient>,
{
    Ok((|| {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.discharge_coefficient != payload.new_discharge_coefficient && !(!payload.new_discharge_coefficient.is_finite() || payload.new_discharge_coefficient <= 0.0) => {
            vec![vocabulary::change_infiltration_discharge_coefficient(payload.id, item.discharge_coefficient)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
