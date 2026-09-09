//! 🔺️ Sparse diff builder for `CreateShadingSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateShadingSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.shading_surfaces.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Shading surface {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A shading surface name must not be blank.", [payload.id.0.to_string()]);
    }
    if payload.vertices_m.len() < 3 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A shading polygon needs at least three vertices, got {}.", payload.vertices_m.len()), [payload.id.0.to_string()]);
    }
    if payload.transmittance_schedule_id.is_some_and(|schedule| !base.model.schedules.contains(schedule)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "The named transmittance schedule is not defined by this model.", [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    let position = model.shading_surfaces.iter().position(|item| item.id > payload.id).unwrap_or(model.shading_surfaces.len());
    model.shading_surfaces.insert(position, crate::model::ShadingSurface { id: payload.id, name: payload.name.clone(), vertices_m: payload.vertices_m.clone(), transmittance_schedule_id: payload.transmittance_schedule_id });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
