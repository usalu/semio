//! 🔺️ Sparse diff builder for `ChangeMaterialSpecificHeat` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MaterialPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialSpecificHeat, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_specific_heat_j_kg_k.is_finite() || payload.new_specific_heat_j_kg_k <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material {}: specific heat (J/kg·K) must be a positive finite value, got {}.", payload.id.0, payload.new_specific_heat_j_kg_k), [payload.id.0.to_string()]);
    }
    if existing.specific_heat_j_kg_k == payload.new_specific_heat_j_kg_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Material {} already carries this specific heat (J/kg·K): {}.", payload.id.0, payload.new_specific_heat_j_kg_k));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { materials: Rows::modifying(MaterialPatch { specific_heat_j_kg_k: Some(payload.new_specific_heat_j_kg_k), ..MaterialPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
