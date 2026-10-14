//! 🔺️ Sparse diff builder for `ChangeSurfaceMultiplier` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSurfaceMultiplier, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_multiplier == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Surface {} needs at least one instance.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.multiplier == payload.new_multiplier {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Surface {} already has this multiplier.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { surfaces: Rows::modifying(SurfacePatch { multiplier: Some(payload.new_multiplier), ..SurfacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
