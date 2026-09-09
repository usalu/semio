//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemCollectorArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemCollectorArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_collector_area_m2.is_finite() || payload.new_collector_area_m2 <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Solar thermal system {}: collector area (m²) must be a positive finite value, got {}.", payload.id.0, payload.new_collector_area_m2), [payload.id.0.to_string()]);
    }
    if existing.collector_area_m2 == payload.new_collector_area_m2 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solar thermal system {} already carries this collector area (m²): {}.", payload.id.0, payload.new_collector_area_m2));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.solar_thermal_systems.iter_mut().find(|item| item.id == payload.id) {
        item.collector_area_m2 = payload.new_collector_area_m2;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
