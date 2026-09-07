//! 🔺️ Sparse diff builder for `ReplaceShadingSurfaceVertices` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceShadingSurfaceVertices, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shading surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_vertices_m.len() < 3 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A shading polygon needs at least three vertices, got {}.", payload.new_vertices_m.len()), [payload.id.0.to_string()]);
    }
    if existing.vertices_m == payload.new_vertices_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shading surface {} already has this polygon.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.shading_surfaces.iter_mut().find(|item| item.id == payload.id) {
        item.vertices_m = payload.new_vertices_m.clone();
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
