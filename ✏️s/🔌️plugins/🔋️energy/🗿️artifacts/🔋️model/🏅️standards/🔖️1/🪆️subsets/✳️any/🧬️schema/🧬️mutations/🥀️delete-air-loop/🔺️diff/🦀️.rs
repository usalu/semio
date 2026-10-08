//! 🔺️ Sparse diff builder for `DeleteAirLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelAirLoopPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteAirLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.outdoor_air_systems.iter().any(|system| system.air_loop_id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Air loop {} still serves an outdoor air system.", payload.id.0), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { air_loops: Rows::removing(&base.model.air_loops, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
