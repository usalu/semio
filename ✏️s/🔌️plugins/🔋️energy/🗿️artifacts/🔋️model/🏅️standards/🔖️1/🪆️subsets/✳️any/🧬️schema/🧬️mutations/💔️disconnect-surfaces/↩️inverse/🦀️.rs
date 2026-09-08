//! ↩️ Inverse for `DisconnectSurfaces` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DisconnectSurfaces, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.adjacency_pairs.iter().find(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) {
        Some(existing) => vec![vocabulary::connect_surfaces(existing.surface_a_id, existing.surface_b_id)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
