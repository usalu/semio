//! ↩️ Inverse for `DeleteRefrigerationSystem` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteRefrigerationSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.refrigeration_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.refrigeration_systems[index];
    vec![vocabulary::create_refrigeration_system(index as u32, existing.id, existing.case_count, existing.design_load_w, existing.defrost_schedule_id)]
}
//#endregion 🔖️Inverse
