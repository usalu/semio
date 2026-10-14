//! 🔺️ Sparse diff builder for `ChangePlantLoopReturnTemperature` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, PlantLoopConfigPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePlantLoopReturnTemperature, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_return_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.new_return_temperature_c) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A return temperature must lie between -100.0 and 300.0, got {}.", payload.new_return_temperature_c), [payload.id.0.to_string()]);
    }
    if existing.return_temperature_c == payload.new_return_temperature_c {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Plant loop {} already has that return temperature.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { plant_loops: Rows::modifying(PlantLoopConfigPatch { return_temperature_c: Some(payload.new_return_temperature_c), ..PlantLoopConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
