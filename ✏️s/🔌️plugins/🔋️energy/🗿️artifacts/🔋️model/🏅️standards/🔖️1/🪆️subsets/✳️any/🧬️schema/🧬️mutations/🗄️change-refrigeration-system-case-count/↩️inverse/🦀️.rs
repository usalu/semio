//! ↩️ Inverse for `ChangeRefrigerationSystemCaseCount` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeRefrigerationSystemCaseCount, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.case_count != payload.new_case_count && payload.new_case_count != 0 => vec![vocabulary::change_refrigeration_system_case_count(payload.id, item.case_count)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
