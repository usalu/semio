//! ↩️ Inverse for `ChangeMaterialSpecificHeat` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMaterialSpecificHeat, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.specific_heat_j_kg_k != payload.new_specific_heat_j_kg_k && !(!payload.new_specific_heat_j_kg_k.is_finite() || payload.new_specific_heat_j_kg_k <= 0.0) => vec![vocabulary::change_material_specific_heat(payload.id, item.specific_heat_j_kg_k)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
