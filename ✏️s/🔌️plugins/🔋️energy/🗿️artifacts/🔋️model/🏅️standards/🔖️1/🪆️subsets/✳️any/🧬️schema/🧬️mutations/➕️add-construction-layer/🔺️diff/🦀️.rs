//! 🔺️ Sparse diff builder for `AddConstructionLayer` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddConstructionLayer, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.materials.iter().any(|material| material.id == payload.material_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.material_id.0), [payload.material_id.0.to_string()]);
    }
    if payload.index as usize > existing.layer_material_ids.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Construction {} has {} layers, so index {} is past its end.", payload.id.0, existing.layer_material_ids.len(), payload.index), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(construction) = model.constructions.iter_mut().find(|item| item.id == payload.id) {
        construction.layer_material_ids.insert(payload.index as usize, payload.material_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
