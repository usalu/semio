//! 🔺️ Sparse diff builder for `DeleteSetpointManager` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SetpointManagerPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteSetpointManager, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.setpoint_managers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Setpoint manager {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;

    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { setpoint_managers: Rows::removing(&base.model.setpoint_managers, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
