//! ↩️ Inverse for `CreateSurface` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateSurface, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.surfaces.iter().any(|item| item.id == payload.id) || crate::model::OutsideBoundary::from_parts(payload.boundary, payload.interzone_surface_id).is_none() || payload.name.trim().is_empty() || !base.model.zones.iter().any(|item| item.id == payload.zone_id) || !base.model.constructions.iter().any(|item| item.id == payload.construction_id) || payload.vertices_m.len() < 3 || payload.interzone_surface_id.is_some_and(|partner| partner == payload.id || !base.model.surfaces.iter().any(|item| item.id == partner)) || payload.multiplier == 0 || false {
        return Vec::new();
    }
    vec![vocabulary::delete_surface(payload.id)]
}
//#endregion 🔖️Inverse
