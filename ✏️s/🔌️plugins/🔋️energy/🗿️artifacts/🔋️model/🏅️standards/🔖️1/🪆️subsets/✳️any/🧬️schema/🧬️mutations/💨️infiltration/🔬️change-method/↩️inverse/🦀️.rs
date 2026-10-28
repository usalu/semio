//! ↩️ Inverse for `ChangeInfiltrationMethod` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeInfiltrationMethod, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeInfiltrationMethod>,
{
    Ok((|| {
    match base.model.infiltrations.iter().find(|item| item.id == payload.id) {
        Some(item) if item.method != payload.new_method => vec![vocabulary::change_infiltration_method(payload.id, item.method)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
