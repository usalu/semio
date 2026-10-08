//! 🔺️ Sparse diff builder for `ConnectSurfaces` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, AdjacencyPairPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectSurfaces, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.surface_a_id == payload.surface_b_id {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A surface is not adjacent to itself.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    }
    if !base.model.surfaces.iter().any(|item| item.id == payload.surface_a_id) || !base.model.surfaces.iter().any(|item| item.id == payload.surface_b_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "An adjacency pair joins two surfaces that already exist.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    }
    if base.model.adjacency_pairs.iter().any(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "These two surfaces are already adjacent.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    }
    let pair = crate::model::AdjacencyPair { surface_a_id: payload.surface_a_id, surface_b_id: payload.surface_b_id };
    let position = base.model.adjacency_pairs.iter().position(|item| (item.surface_a_id, item.surface_b_id) > (pair.surface_a_id, pair.surface_b_id)).unwrap_or(base.model.adjacency_pairs.len());
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { adjacency_pairs: Rows::inserting(position, pair), ..Default::default() }))
}
//#endregion 🔖️Diff
