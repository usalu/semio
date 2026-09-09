//! 🔺️ Sparse diff builder for `DeleteSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.surfaces.iter().any(|item| item.outside_boundary_condition.interzone_partner() == Some(payload.id)) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Surface {} is another surface's interzone partner; change that boundary condition first.", payload.id.0), [payload.id.0.to_string()]);
    }
    let fenestrations = base.model.fenestrations.iter().filter(|item| item.surface_id == payload.id).count();
    let pairs = base.model.adjacency_pairs.iter().filter(|item| item.surface_a_id == payload.id || item.surface_b_id == payload.id).count();
    let mut model = base.model.clone();
    model.surfaces.retain(|item| item.id != payload.id);
    model.fenestrations.retain(|item| item.surface_id != payload.id);
    model.adjacency_pairs.retain(|item| item.surface_a_id != payload.id && item.surface_b_id != payload.id);
    let outcome = protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model));
    if fenestrations + pairs == 0 {
        return outcome;
    }
    outcome.info("mutation.cascade", format!("Deleting surface {} also removed {fenestrations} fenestration(s) and {pairs} adjacency pair(s).", payload.id.0))
}
//#endregion 🔖️Diff
