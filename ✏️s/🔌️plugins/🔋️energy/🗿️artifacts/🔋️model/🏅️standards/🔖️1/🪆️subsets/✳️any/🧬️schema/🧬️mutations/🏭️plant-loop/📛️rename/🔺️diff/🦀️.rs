//! 🔺️ Sparse diff builder for `RenamePlantLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, PlantLoopConfigPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenamePlantLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A plant loop name must not be blank.".to_string(), [payload.id.0.to_string()]);
    }
    if base.model.plant_loops.iter().any(|item| item.id != payload.id && item.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another plant loop is already named {:?}.", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Plant loop {} already has that name.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { plant_loops: Rows::modifying(PlantLoopConfigPatch { name: Some(payload.new_name.clone()), ..PlantLoopConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
