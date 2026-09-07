//! ↩️ Inverse for `DisconnectSurfaces` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DisconnectSurfaces, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.adjacency_pairs.iter().find(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) {
        Some(existing) => vec![vocabulary::connect_surfaces(existing.surface_a_id, existing.surface_b_id)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
