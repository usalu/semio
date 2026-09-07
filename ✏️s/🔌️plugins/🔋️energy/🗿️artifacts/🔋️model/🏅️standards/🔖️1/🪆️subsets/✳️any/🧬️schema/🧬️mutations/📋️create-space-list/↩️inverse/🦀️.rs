//! ↩️ Inverse for `CreateSpaceList` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateSpaceList, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.space_lists.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.space_lists.len() || payload.space_ids.iter().any(|candidate| !base.model.spaces.iter().any(|row| row.id == *candidate)) {
        return Vec::new();
    }
    vec![vocabulary::delete_space_list(payload.id)]
}
//#endregion 🔖️Inverse
