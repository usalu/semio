//! 🔺️ Sparse diff builder for `ReplaceFenestrationVertices` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceFenestrationVertices, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_vertices_m.is_empty() && payload.new_vertices_m.len() < 3 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("An aperture polygon is either empty — deriving the rectangle from area, height and sill — or a ring of at least three vertices, got {}.", payload.new_vertices_m.len()), [payload.id.0.to_string()]);
    }
    if !payload.new_vertices_m.iter().flatten().all(|coordinate| coordinate.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {}: every polygon coordinate must be finite.", payload.id.0), [payload.id.0.to_string()]);
    }
    if let Some(host) = base.model.surfaces.iter().find(|item| item.id == existing.surface_id) {
        if !crate::geometry::polygon_lies_on_plane(&payload.new_vertices_m, &host.vertices_m, crate::model::FENESTRATION_PLANE_TOLERANCE_M) {
            return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {}: the polygon must lie in host surface {}'s own plane.", payload.id.0, host.id.0), [payload.id.0.to_string()]);
        }
    }
    if existing.vertices_m == payload.new_vertices_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} already has this polygon.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.vertices_m = payload.new_vertices_m.clone();
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
