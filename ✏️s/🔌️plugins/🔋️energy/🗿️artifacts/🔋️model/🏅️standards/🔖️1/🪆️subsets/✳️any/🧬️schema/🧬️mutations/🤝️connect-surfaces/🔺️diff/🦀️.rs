//! 🔺️ Sparse diff builder for `ConnectSurfaces` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectSurfaces, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.surface_a_id == payload.surface_b_id {
        return protocol::MutationOutcome::error("mutation.invariant", "A surface is not adjacent to itself.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    }
    if !base.model.surfaces.iter().any(|item| item.id == payload.surface_a_id) || !base.model.surfaces.iter().any(|item| item.id == payload.surface_b_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "An adjacency pair joins two surfaces that already exist.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    }
    if base.model.adjacency_pairs.iter().any(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", "These two surfaces are already adjacent.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    let pair = crate::model::AdjacencyPair { surface_a_id: payload.surface_a_id, surface_b_id: payload.surface_b_id };
    let position = model.adjacency_pairs.iter().position(|item| (item.surface_a_id, item.surface_b_id) > (pair.surface_a_id, pair.surface_b_id)).unwrap_or(model.adjacency_pairs.len());
    model.adjacency_pairs.insert(position, pair);
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
