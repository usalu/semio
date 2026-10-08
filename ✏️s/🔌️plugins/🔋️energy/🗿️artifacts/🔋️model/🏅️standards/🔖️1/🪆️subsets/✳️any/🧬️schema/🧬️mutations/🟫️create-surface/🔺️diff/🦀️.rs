//! 🔺️ Sparse diff builder for `CreateSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.surfaces.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Surface {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A surface name must not be blank.", [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|item| item.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.id.0.to_string()]);
    }
    if !base.model.constructions.iter().any(|item| item.id == payload.construction_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.construction_id.0), [payload.id.0.to_string()]);
    }
    if payload.vertices_m.len() < 3 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A surface polygon needs at least three vertices, got {}.", payload.vertices_m.len()), [payload.id.0.to_string()]);
    }
    if payload.interzone_surface_id.is_some_and(|partner| partner == payload.id || !base.model.surfaces.iter().any(|item| item.id == partner)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "An interzone partner must be another surface that already exists.", [payload.id.0.to_string()]);
    }
    if payload.multiplier == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Surface {} needs at least one instance.", payload.id.0), [payload.id.0.to_string()]);
    }
    let Some(boundary) = crate::model::OutsideBoundary::from_parts(payload.boundary, payload.interzone_surface_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "An interzone boundary names exactly one partner surface, and every other boundary names none.", [payload.id.0.to_string()]);
    };
    let position = payload.index.map_or_else(|| base.model.surfaces.iter().position(|item| item.id > payload.id).unwrap_or(base.model.surfaces.len()), |index| index as usize);
    if position > base.model.surfaces.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} surfaces.", position, base.model.surfaces.len()), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { surfaces: Rows::inserting(position, crate::model::Surface {
            id: payload.id,
            name: payload.name.clone(),
            zone_id: payload.zone_id,
            class: payload.class,
            vertices_m: payload.vertices_m.clone(),
            construction_id: payload.construction_id,
            outside_boundary_condition: boundary,
            sun_exposed: payload.sun_exposed,
            wind_exposed: payload.wind_exposed,
            multiplier: payload.multiplier,
        }), ..Default::default() }))
}
//#endregion 🔖️Diff
