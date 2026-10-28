//! 🔺️ Sparse diff builder for `ReplaceSurfaceVertices` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch, SurfacePatch};
use crate::diff_patch::{Rows};
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceSurfaceVertices, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_vertices_m.len() < 3 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A surface polygon needs at least three vertices, got {}.", payload.new_vertices_m.len()), [payload.id.0.to_string()]);
    }
    if existing.vertices_m == payload.new_vertices_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Surface {} already has this polygon.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { surfaces: Rows::modifying(SurfacePatch { vertices_m: Some(payload.new_vertices_m.clone()), ..SurfacePatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
