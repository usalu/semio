//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemCollectorArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SolarThermalConfigPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemCollectorArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_collector_area_m2.is_finite() || payload.new_collector_area_m2 <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Solar thermal system {}: collector area (m²) must be a positive finite value, got {}.", payload.id.0, payload.new_collector_area_m2), [payload.id.0.to_string()]);
    }
    if existing.collector_area_m2 == payload.new_collector_area_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Solar thermal system {} already carries this collector area (m²): {}.", payload.id.0, payload.new_collector_area_m2));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { solar_thermal_systems: Rows::modifying(SolarThermalConfigPatch { collector_area_m2: Some(payload.new_collector_area_m2), ..SolarThermalConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
