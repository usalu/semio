//! 🔺️ Sparse diff builder for `CreateMaterial` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateMaterial, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.materials.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Material {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.materials.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} materials.", payload.index, base.model.materials.len()), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.materials.insert(payload.index as usize, crate::model::Material { id: payload.id, name: payload.name.clone(), thickness_m: payload.thickness_m, conductivity_w_m_k: payload.conductivity_w_m_k, density_kg_m3: payload.density_kg_m3, specific_heat_j_kg_k: payload.specific_heat_j_kg_k, thermal_absorptance: payload.thermal_absorptance, solar_absorptance: payload.solar_absorptance, visible_absorptance: payload.visible_absorptance });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
