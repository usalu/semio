//! ↩️ Inverse for `ChangeMaterialThermalAbsorptance` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeMaterialThermalAbsorptance, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.thermal_absorptance != payload.new_thermal_absorptance && !(!(0.0..=1.0).contains(&payload.new_thermal_absorptance)) => vec![vocabulary::change_material_thermal_absorptance(payload.id, item.thermal_absorptance)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
