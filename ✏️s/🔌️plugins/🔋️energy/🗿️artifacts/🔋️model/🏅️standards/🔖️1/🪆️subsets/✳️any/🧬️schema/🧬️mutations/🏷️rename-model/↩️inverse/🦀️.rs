//! ↩️ Inverse for `RenameModel` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RenameModel, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.new_name.trim().is_empty() || base.model.name == payload.new_name {
        return Vec::new();
    }
    vec![vocabulary::rename_model(base.model.name.clone())]
}
//#endregion 🔖️Inverse
