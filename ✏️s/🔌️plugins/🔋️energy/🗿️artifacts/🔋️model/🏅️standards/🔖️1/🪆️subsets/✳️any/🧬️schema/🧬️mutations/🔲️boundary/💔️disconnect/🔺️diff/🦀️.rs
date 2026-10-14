//! 🔺️ Sparse diff builder for `DisconnectSurfaces` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, AdjacencyPairPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectSurfaces, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.adjacency_pairs.iter().find(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id))
    else {
        return protocol::MutationOutcome::error("mutation.target-missing", "These two surfaces are not adjacent.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { adjacency_pairs: Rows::<AdjacencyPairPatch>::removing_where(&base.model.adjacency_pairs, |item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)), ..Default::default() }))
}
//#endregion 🔖️Diff
