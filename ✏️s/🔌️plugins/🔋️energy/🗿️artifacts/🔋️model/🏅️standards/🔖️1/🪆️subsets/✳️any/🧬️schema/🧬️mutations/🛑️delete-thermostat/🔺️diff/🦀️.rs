//! 🔺️ Sparse diff builder for `DeleteThermostat` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ThermostatPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteThermostat, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermostats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermostat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;

    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { thermostats: Rows::removing(&base.model.thermostats, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
