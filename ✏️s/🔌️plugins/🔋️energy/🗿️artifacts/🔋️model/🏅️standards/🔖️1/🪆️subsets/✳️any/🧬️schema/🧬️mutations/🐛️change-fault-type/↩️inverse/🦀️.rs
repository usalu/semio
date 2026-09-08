//! ↩️ Inverse for `ChangeFaultType` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFaultType, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.faults.iter().find(|item| item.id == payload.id) {
        Some(item) if item.fault_type != payload.new_fault_type => vec![vocabulary::change_fault_type(payload.id, item.fault_type)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
