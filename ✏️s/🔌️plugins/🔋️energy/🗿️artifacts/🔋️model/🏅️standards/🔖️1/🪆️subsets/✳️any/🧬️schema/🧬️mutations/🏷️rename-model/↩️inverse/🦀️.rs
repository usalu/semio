//! ↩️ Inverse for `RenameModel` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RenameModel, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.new_name.trim().is_empty() || base.model.name == payload.new_name {
        return Vec::new();
    }
    vec![vocabulary::rename_model(base.model.name.clone())]
}
//#endregion 🔖️Inverse
