//! 🔺️ Sparse diff builder for `RemoveConstructionLayer` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ConstructionPatch, ListEdit, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveConstructionLayer, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.layer_material_ids.get(payload.index as usize).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} has no layer at index {}.", payload.id.0, payload.index), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { constructions: Rows::modifying(ConstructionPatch { layer_material_ids: ListEdit::removing_index(&existing.layer_material_ids, payload.index as usize), ..ConstructionPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
