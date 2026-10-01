//! 🔺️ Sparse diff builder for `ChangeFenestrationSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.surfaces.iter().any(|item| item.id == payload.new_surface_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.new_surface_id.0), [payload.id.0.to_string()]);
    }
    if existing.surface_id == payload.new_surface_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} already has this host surface.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.surface_id = payload.new_surface_id;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
