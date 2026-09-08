//! ↩️ Inverse for `RenameSpaceList` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RenameSpaceList, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.space_lists.iter().find(|item| item.id == payload.id) {
        Some(item) if item.name != payload.new_name && !payload.new_name.trim().is_empty() && !base.model.space_lists.iter().any(|other| other.id != payload.id && other.name == payload.new_name) => vec![vocabulary::rename_space_list(payload.id, item.name.clone())],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
