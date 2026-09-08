//! 🔺️ Sparse diff builder for `ChangeSurfaceBoundaryCondition` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSurfaceBoundaryCondition, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let Some(boundary) = crate::model::OutsideBoundary::from_parts(payload.new_boundary, payload.new_interzone_surface_id) else {
        return protocol::MutationOutcome::error("mutation.invariant", "An interzone boundary names exactly one partner surface, and every other boundary names none.", [payload.id.0.to_string()]);
    };
    if payload.new_interzone_surface_id.is_some_and(|partner| partner == payload.id || !base.model.surfaces.iter().any(|item| item.id == partner)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "An interzone partner must be another surface that already exists.", [payload.id.0.to_string()]);
    }
    if existing.outside_boundary_condition == boundary {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Surface {} already has this boundary condition.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.surfaces.iter_mut().find(|item| item.id == payload.id) {
        item.outside_boundary_condition = boundary;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
