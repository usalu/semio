//! 🔺️ Sparse diff builder for `ChangeSurfaceConstruction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSurfaceConstruction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.constructions.iter().any(|item| item.id == payload.new_construction_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.new_construction_id.0), [payload.id.0.to_string()]);
    }
    if existing.construction_id == payload.new_construction_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Surface {} already has this construction.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { surfaces: Rows::modifying(SurfacePatch { construction_id: Some(payload.new_construction_id), ..SurfacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
