//! ↩️ Inverse for `CreateMaterial` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateMaterial, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.materials.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.materials.len() {
        return Vec::new();
    }
    vec![vocabulary::delete_material(payload.id)]
}
//#endregion 🔖️Inverse
