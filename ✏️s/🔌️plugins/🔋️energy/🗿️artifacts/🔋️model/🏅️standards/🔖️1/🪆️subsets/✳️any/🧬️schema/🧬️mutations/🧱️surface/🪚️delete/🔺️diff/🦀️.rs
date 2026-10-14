//! 🔺️ Sparse diff builder for `DeleteSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, AdjacencyPairPatch, FenestrationPatch, ModelPatch, Rows, SurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.surfaces.iter().any(|item| item.outside_boundary_condition.interzone_partner() == Some(payload.id)) {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Surface {} is another surface's interzone partner; change that boundary condition first.", payload.id.0), [payload.id.0.to_string()]);
    }
    let fenestrations = base.model.fenestrations.iter().filter(|item| item.surface_id == payload.id).count();
    let pairs = base.model.adjacency_pairs.iter().filter(|item| item.surface_a_id == payload.id || item.surface_b_id == payload.id).count();
    let outcome = protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch {
        surfaces: Rows::<SurfacePatch>::removing(&base.model.surfaces, &payload.id),
        fenestrations: Rows::<FenestrationPatch>::removing_where(&base.model.fenestrations, |item| item.surface_id == payload.id),
        adjacency_pairs: Rows::<AdjacencyPairPatch>::removing_where(&base.model.adjacency_pairs, |item| item.surface_a_id == payload.id || item.surface_b_id == payload.id),
        ..Default::default()
    }));
    if fenestrations + pairs == 0 {
        return outcome;
    }
    outcome.info("mutation.cascade", format!("Deleting surface {} also removed {fenestrations} fenestration(s) and {pairs} adjacency pair(s).", payload.id.0))
}
//#endregion 🔖️Diff
