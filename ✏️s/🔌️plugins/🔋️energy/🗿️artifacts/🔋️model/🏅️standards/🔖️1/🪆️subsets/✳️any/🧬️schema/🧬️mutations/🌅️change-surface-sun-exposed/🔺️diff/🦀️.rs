//! 🔺️ Sparse diff builder for `ChangeSurfaceSunExposed` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSurfaceSunExposed, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.sun_exposed == payload.new_sun_exposed {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Surface {} already has this sun exposure.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { surfaces: Rows::modifying(SurfacePatch { sun_exposed: Some(payload.new_sun_exposed), ..SurfacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
