//! ↩️ Inverse for `ChangeGlazingMaterialInfraredEmissivity` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeGlazingMaterialInfraredEmissivity, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let valid = [payload.new_infrared_emissivity_front, payload.new_infrared_emissivity_back].iter().all(|value| value.is_finite() && (0.0..=1.0).contains(value));
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if valid && (item.infrared_emissivity_front != payload.new_infrared_emissivity_front || item.infrared_emissivity_back != payload.new_infrared_emissivity_back) => {
            vec![vocabulary::change_glazing_material_infrared_emissivity(payload.id, item.infrared_emissivity_front, item.infrared_emissivity_back)]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
