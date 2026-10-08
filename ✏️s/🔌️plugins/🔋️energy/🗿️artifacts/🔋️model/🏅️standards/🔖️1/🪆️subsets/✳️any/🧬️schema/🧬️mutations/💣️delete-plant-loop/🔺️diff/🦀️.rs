//! 🔺️ Sparse diff builder for `DeletePlantLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, PlantLoopConfigPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeletePlantLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;

    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { plant_loops: Rows::removing(&base.model.plant_loops, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
