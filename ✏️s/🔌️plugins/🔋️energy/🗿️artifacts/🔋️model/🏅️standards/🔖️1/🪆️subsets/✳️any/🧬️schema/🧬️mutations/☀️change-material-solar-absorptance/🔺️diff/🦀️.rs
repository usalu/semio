//! 🔺️ Sparse diff builder for `ChangeMaterialSolarAbsorptance` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMaterialSolarAbsorptance, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_solar_absorptance) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Material {}: solar absorptance must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_solar_absorptance), [payload.id.0.to_string()]);
    }
    if existing.solar_absorptance == payload.new_solar_absorptance {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material {} already carries this solar absorptance: {}.", payload.id.0, payload.new_solar_absorptance));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.materials.iter_mut().find(|item| item.id == payload.id) {
        item.solar_absorptance = payload.new_solar_absorptance;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
