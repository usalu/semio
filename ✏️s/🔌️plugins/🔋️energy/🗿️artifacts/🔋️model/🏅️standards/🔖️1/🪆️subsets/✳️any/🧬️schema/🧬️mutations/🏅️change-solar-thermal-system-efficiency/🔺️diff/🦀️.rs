//! 🔺️ Sparse diff builder for `ChangeSolarThermalSystemEfficiency` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSolarThermalSystemEfficiency, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Solar thermal system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(payload.new_efficiency > 0.0 && payload.new_efficiency <= 1.0) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Solar thermal system {}: collector efficiency must be a fraction in (0, 1], got {}.", payload.id.0, payload.new_efficiency), [payload.id.0.to_string()]);
    }
    if existing.efficiency == payload.new_efficiency {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Solar thermal system {} already carries this collector efficiency: {}.", payload.id.0, payload.new_efficiency));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.solar_thermal_systems.iter_mut().find(|item| item.id == payload.id) {
        item.efficiency = payload.new_efficiency;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
