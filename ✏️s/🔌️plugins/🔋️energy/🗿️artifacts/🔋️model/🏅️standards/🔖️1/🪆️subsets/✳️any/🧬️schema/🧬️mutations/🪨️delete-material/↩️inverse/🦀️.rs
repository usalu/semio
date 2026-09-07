//! ↩️ Inverse for `DeleteMaterial` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteMaterial, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.materials.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.constructions.iter().any(|construction| construction.layer_material_ids.contains(&payload.id)) {
        return Vec::new();
    }
    let existing = &base.model.materials[index];
    vec![vocabulary::create_material(index as u32, existing.id, existing.name.clone(), existing.thickness_m, existing.conductivity_w_m_k, existing.density_kg_m3, existing.specific_heat_j_kg_k, existing.thermal_absorptance, existing.solar_absorptance, existing.visible_absorptance)]
}
//#endregion 🔖️Inverse
