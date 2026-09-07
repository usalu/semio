//! 🔺️ Sparse diff builder for `ReorderConstructionLayers` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ReorderConstructionLayers, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let mut wanted: Vec<u32> = payload.new_layer_material_ids.iter().map(|id| id.0).collect();
    let mut held: Vec<u32> = existing.layer_material_ids.iter().map(|id| id.0).collect();
    wanted.sort_unstable();
    held.sort_unstable();
    if wanted != held {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Construction {}'s layers can only be reordered, not exchanged.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.layer_material_ids == payload.new_layer_material_ids {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Construction {} already holds its layers in this order.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(construction) = model.constructions.iter_mut().find(|item| item.id == payload.id) {
        construction.layer_material_ids = payload.new_layer_material_ids.clone();
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
