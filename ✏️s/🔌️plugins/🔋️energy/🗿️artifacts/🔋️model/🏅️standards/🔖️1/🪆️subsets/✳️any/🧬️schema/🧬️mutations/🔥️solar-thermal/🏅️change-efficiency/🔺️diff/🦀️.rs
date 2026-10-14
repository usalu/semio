//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemEfficiency` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SolarThermalConfigPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemEfficiency, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(payload.new_efficiency > 0.0 && payload.new_efficiency <= 1.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Solar thermal system {}: collector efficiency must be a fraction in (0, 1], got {}.", payload.id.0, payload.new_efficiency), [payload.id.0.to_string()]);
    }
    if existing.efficiency == payload.new_efficiency {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Solar thermal system {} already carries this collector efficiency: {}.", payload.id.0, payload.new_efficiency));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { solar_thermal_systems: Rows::modifying(SolarThermalConfigPatch { efficiency: Some(payload.new_efficiency), ..SolarThermalConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
