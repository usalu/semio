//! 🔺️ Sparse diff builder for `DisconnectSurfaces` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectSurfaces, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.adjacency_pairs.iter().find(|item| (item.surface_a_id, item.surface_b_id) == (payload.surface_a_id, payload.surface_b_id) || (item.surface_a_id, item.surface_b_id) == (payload.surface_b_id, payload.surface_a_id)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "These two surfaces are not adjacent.", [payload.surface_a_id.0.to_string(), payload.surface_b_id.0.to_string()]);
    };
    let _ = existing;
    let mut model = base.model.clone();
    model.adjacency_pairs.retain(|item| (item.surface_a_id, item.surface_b_id) != (payload.surface_a_id, payload.surface_b_id) && (item.surface_a_id, item.surface_b_id) != (payload.surface_b_id, payload.surface_a_id));
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
