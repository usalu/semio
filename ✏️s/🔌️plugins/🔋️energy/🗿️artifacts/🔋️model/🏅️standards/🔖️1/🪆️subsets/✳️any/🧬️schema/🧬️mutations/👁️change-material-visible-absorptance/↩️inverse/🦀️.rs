//! ↩️ Inverse for `ChangeMaterialVisibleAbsorptance` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMaterialVisibleAbsorptance, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.visible_absorptance != payload.new_visible_absorptance && !(!(0.0..=1.0).contains(&payload.new_visible_absorptance)) => vec![vocabulary::change_material_visible_absorptance(payload.id, item.visible_absorptance)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
