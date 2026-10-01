//! ↩️ Inverse for `ChangeGlazingMaterialVisibleTransmittance` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeGlazingMaterialVisibleTransmittance, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.visible_transmittance != payload.new_visible_transmittance && payload.new_visible_transmittance.is_finite() && (0.0..=1.0).contains(&payload.new_visible_transmittance) => vec![vocabulary::change_glazing_material_visible_transmittance(payload.id, item.visible_transmittance)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
