//! 🔺️ Sparse diff builder for `RemoveConstructionLayer` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveConstructionLayer, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.layer_material_ids.get(payload.index as usize).is_none() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Construction {} has no layer at index {}.", payload.id.0, payload.index), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(construction) = model.constructions.iter_mut().find(|item| item.id == payload.id) {
        construction.layer_material_ids.remove(payload.index as usize);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
