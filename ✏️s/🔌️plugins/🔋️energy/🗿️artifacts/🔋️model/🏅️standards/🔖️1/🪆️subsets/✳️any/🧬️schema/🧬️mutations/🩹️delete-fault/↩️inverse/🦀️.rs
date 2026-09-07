//! ↩️ Inverse for `DeleteFault` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteFault, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.faults.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.faults[index];
    vec![vocabulary::create_fault(index as u32, existing.id, existing.target_equipment_id, existing.fault_type, existing.severity, existing.start_schedule_id)]
}
//#endregion 🔖️Inverse
