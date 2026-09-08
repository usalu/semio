//! 🔺️ Sparse diff builder for `ChangeMaterialThermalAbsorptance` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialThermalAbsorptance, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_thermal_absorptance) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Material {}: thermal absorptance must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_thermal_absorptance), [payload.id.0.to_string()]);
    }
    if existing.thermal_absorptance == payload.new_thermal_absorptance {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material {} already carries this thermal absorptance: {}.", payload.id.0, payload.new_thermal_absorptance));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.materials.iter_mut().find(|item| item.id == payload.id) {
        item.thermal_absorptance = payload.new_thermal_absorptance;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
