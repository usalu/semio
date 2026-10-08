//! 🔺️ Sparse diff builder for `CreateShadingSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ShadingSurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateShadingSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.shading_surfaces.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Shading surface {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A shading surface name must not be blank.", [payload.id.0.to_string()]);
    }
    if payload.vertices_m.len() < 3 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A shading polygon needs at least three vertices, got {}.", payload.vertices_m.len()), [payload.id.0.to_string()]);
    }
    if payload.transmittance_schedule_id.is_some_and(|schedule| !base.model.schedules.contains(schedule)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "The named transmittance schedule is not defined by this model.", [payload.id.0.to_string()]);
    }
    let position = payload.index.map_or_else(|| base.model.shading_surfaces.iter().position(|item| item.id > payload.id).unwrap_or(base.model.shading_surfaces.len()), |index| index as usize);
    if position > base.model.shading_surfaces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} shading_surfaces.", position, base.model.shading_surfaces.len()), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { shading_surfaces: Rows::inserting(position, crate::model::ShadingSurface { id: payload.id, name: payload.name.clone(), vertices_m: payload.vertices_m.clone(), transmittance_schedule_id: payload.transmittance_schedule_id }), ..Default::default() }))
}
//#endregion 🔖️Diff
