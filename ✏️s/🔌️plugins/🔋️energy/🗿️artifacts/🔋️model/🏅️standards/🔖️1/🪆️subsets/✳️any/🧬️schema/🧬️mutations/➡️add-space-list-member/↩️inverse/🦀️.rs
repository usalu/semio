//! ↩️ Inverse for `AddSpaceListMember` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::AddSpaceListMember, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.space_lists.iter().find(|item| item.id == payload.id) {
        Some(item) if !item.space_ids.contains(&payload.space_id) && payload.index as usize <= item.space_ids.len() && base.model.spaces.iter().any(|row| row.id == payload.space_id) => vec![vocabulary::remove_space_list_member(payload.id, payload.space_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
