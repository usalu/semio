//! ↩️ Inverse for `ChangeFaultType` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeFaultType, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeFaultType>,
{
    Ok((|| {
    match base.model.faults.iter().find(|item| item.id == payload.id) {
        Some(item) if item.fault_type != payload.new_fault_type => vec![vocabulary::change_fault_type(payload.id, item.fault_type)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
