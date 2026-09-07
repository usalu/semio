//! ↩️ Inverse for `RemoveSpaceListMember` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RemoveSpaceListMember, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(item) = base.model.space_lists.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(position) = item.space_ids.iter().position(|candidate| *candidate == payload.space_id) else {
        return Vec::new();
    };
    vec![vocabulary::add_space_list_member(payload.id, position as u32, payload.space_id)]
}
//#endregion 🔖️Inverse
