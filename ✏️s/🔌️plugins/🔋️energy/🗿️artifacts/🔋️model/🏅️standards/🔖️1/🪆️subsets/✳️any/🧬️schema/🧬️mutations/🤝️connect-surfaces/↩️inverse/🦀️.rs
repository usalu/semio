//! ↩️ Inverse for `ConnectSurfaces` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ConnectSurfaces, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.surface_a_id == payload.surface_b_id
        || !base.model.surfaces.iter().any(|item| item.id == payload.surface_a_id)
        || !base.model.surfaces.iter().any(|item| item.id == payload.surface_b_id)
        || base.model.adjacency_pairs.iter().any(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id))
    {
        return Vec::new();
    }
    vec![vocabulary::disconnect_surfaces(payload.surface_a_id, payload.surface_b_id)]
}
//#endregion 🔖️Inverse
