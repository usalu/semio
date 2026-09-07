//! ↩️ Inverse for `ChangeFaultType` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFaultType, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.faults.iter().find(|item| item.id == payload.id) {
        Some(item) if item.fault_type != payload.new_fault_type => vec![vocabulary::change_fault_type(payload.id, item.fault_type)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
