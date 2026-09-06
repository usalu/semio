//! 🔺️ Sparse diff builder for `UpdateGroundTemperature` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateGroundTemperature, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.building_surface_c.len() != 12 || payload.shallow_c.len() != 12 {
        return protocol::MutationOutcome::error("mutation.invalid-payload", format!("Ground temperatures need twelve monthly values each, got {} building-surface and {} shallow.", payload.building_surface_c.len(), payload.shallow_c.len()), Vec::<String>::new());
    }
    let mut building_surface_c = [0.0f64; 12];
    let mut shallow_c = [0.0f64; 12];
    building_surface_c.copy_from_slice(&payload.building_surface_c);
    shallow_c.copy_from_slice(&payload.shallow_c);
    let ground = crate::model::GroundTemperatureConfig { building_surface_c, shallow_c, deep_c: payload.deep_c };
    if base.model.ground_temperature == ground {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The ground-temperature facet already has this value.");
    }
    let mut model = base.model.clone();
    model.ground_temperature = ground;
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
