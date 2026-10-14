//! 🔺️ Sparse diff builder for `ReplaceShadingSurfaceVertices` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ShadingSurfacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceShadingSurfaceVertices, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shading surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_vertices_m.len() < 3 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A shading polygon needs at least three vertices, got {}.", payload.new_vertices_m.len()), [payload.id.0.to_string()]);
    }
    if existing.vertices_m == payload.new_vertices_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shading surface {} already has this polygon.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { shading_surfaces: Rows::modifying(ShadingSurfacePatch { vertices_m: Some(payload.new_vertices_m.clone()), ..ShadingSurfacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
