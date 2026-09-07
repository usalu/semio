//! ↩️ Inverse for `ChangeSurfaceBoundaryCondition` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeSurfaceBoundaryCondition, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let Some(boundary) = crate::model::OutsideBoundary::from_parts(payload.new_boundary, payload.new_interzone_surface_id) else {
        return Vec::new();
    };
    if payload.new_interzone_surface_id.is_some_and(|partner| partner == payload.id || !base.model.surfaces.iter().any(|item| item.id == partner)) || existing.outside_boundary_condition == boundary {
        return Vec::new();
    }
    vec![vocabulary::change_surface_boundary_condition(payload.id, existing.outside_boundary_condition.kind(), existing.outside_boundary_condition.interzone_partner())]
}
//#endregion 🔖️Inverse
