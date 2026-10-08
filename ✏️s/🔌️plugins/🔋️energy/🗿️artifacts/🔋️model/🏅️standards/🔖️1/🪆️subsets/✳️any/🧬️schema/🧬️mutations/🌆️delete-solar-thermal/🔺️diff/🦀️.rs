//! 🔺️ Sparse diff builder for `DeleteSolarThermalSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SolarThermalConfigPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteSolarThermalSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { solar_thermal_systems: Rows::removing(&base.model.solar_thermal_systems, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
