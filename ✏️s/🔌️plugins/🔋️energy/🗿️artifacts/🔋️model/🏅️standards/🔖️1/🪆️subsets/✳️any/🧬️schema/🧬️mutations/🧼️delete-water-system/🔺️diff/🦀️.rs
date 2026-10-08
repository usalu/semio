//! 🔺️ Sparse diff builder for `DeleteWaterSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, WaterSystemConfigPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteWaterSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.water_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Water system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { water_systems: Rows::removing(&base.model.water_systems, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
