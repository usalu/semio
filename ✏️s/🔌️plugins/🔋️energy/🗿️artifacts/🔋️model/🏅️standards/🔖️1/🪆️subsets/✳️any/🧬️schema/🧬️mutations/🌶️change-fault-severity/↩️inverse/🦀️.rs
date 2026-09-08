//! ↩️ Inverse for `ChangeFaultSeverity` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFaultSeverity, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.faults.iter().find(|item| item.id == payload.id) {
        Some(item) if item.severity != payload.new_severity && !(!(0.0..=1.0).contains(&payload.new_severity)) => vec![vocabulary::change_fault_severity(payload.id, item.severity)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
