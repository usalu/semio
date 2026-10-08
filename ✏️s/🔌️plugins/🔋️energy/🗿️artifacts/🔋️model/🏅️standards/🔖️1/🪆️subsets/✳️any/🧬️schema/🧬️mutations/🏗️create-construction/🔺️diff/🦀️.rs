//! 🔺️ Sparse diff builder for `CreateConstruction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ConstructionPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateConstruction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.constructions.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Construction {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.constructions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} constructions.", payload.index, base.model.constructions.len()), [payload.id.0.to_string()]);
    }
    if let Some(missing) = payload.layer_material_ids.iter().find(|id| !base.model.materials.iter().any(|material| material.id == **id)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", missing.0), [missing.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { constructions: Rows::inserting(payload.index as usize, crate::model::Construction { id: payload.id, name: payload.name.clone(), layer_material_ids: payload.layer_material_ids.clone() }), ..Default::default() }))
}
//#endregion 🔖️Diff
