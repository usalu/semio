//! ↩️ Inverse for `CreateConstruction` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateConstruction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.constructions.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.constructions.len() || payload.layer_material_ids.iter().any(|id| !base.model.materials.iter().any(|material| material.id == *id)) {
        return Vec::new();
    }
    vec![vocabulary::delete_construction(payload.id)]
}
//#endregion 🔖️Inverse
