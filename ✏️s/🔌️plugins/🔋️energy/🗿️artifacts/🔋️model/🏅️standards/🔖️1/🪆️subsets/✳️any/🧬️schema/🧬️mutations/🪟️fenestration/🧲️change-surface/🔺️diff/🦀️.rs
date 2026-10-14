//! 🔺️ Sparse diff builder for `ChangeFenestrationSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FenestrationPatch, ModelPatch, Rows};
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
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fenestration {} already has this host surface.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { fenestrations: Rows::modifying(FenestrationPatch { surface_id: Some(payload.new_surface_id), ..FenestrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
