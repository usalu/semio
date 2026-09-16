//! ↩️ Inverse for `ChangeMaterialRoughness` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMaterialRoughness, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.roughness != payload.new_roughness => vec![vocabulary::change_material_roughness(payload.id, item.roughness)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
