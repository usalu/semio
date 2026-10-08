//! 🔺️ Sparse diff builder for `DeleteShadingSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ShadingSurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteShadingSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shading surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { shading_surfaces: Rows::removing(&base.model.shading_surfaces, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
