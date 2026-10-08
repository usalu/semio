//! 🔺️ Sparse diff builder for `RenameShadingSurface` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ShadingSurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameShadingSurface, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shading surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A shading surface name must not be blank.", [payload.id.0.to_string()]);
    }
    if base.model.shading_surfaces.iter().any(|item| item.id != payload.id && item.name == payload.new_name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Another shading surface is already named \"{}\".", payload.new_name), [payload.id.0.to_string()]);
    }
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shading surface {} already has this name.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { shading_surfaces: Rows::modifying(ShadingSurfacePatch { name: Some(payload.new_name.clone()), ..ShadingSurfacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
