//! ↩️ Inverse for `CreateMaterial` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateMaterial, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.materials.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.materials.len() {
        return Vec::new();
    }
    vec![vocabulary::delete_material(payload.id)]
}
//#endregion 🔖️Inverse
