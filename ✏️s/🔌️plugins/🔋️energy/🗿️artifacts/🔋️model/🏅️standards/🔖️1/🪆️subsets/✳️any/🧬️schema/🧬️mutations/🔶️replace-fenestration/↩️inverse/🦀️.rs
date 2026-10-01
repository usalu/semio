//! ↩️ Inverse for `ReplaceFenestrationVertices` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ReplaceFenestrationVertices, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    // ↩️ The same three guards the forward diff refuses on, restated — a refused step has nothing
    // to undo, and restating them keeps this function computed from BASE alone.
    let on_host_plane = match base.model.surfaces.iter().find(|item| item.id == existing.surface_id) {
        Some(host) => crate::geometry::polygon_lies_on_plane(&payload.new_vertices_m, &host.vertices_m, crate::model::FENESTRATION_PLANE_TOLERANCE_M),
        None => true,
    };
    let degenerate = !payload.new_vertices_m.is_empty() && payload.new_vertices_m.len() < 3;
    let finite = payload.new_vertices_m.iter().flatten().all(|coordinate| coordinate.is_finite());
    if degenerate || !finite || !on_host_plane || existing.vertices_m == payload.new_vertices_m {
        return Vec::new();
    }
    vec![vocabulary::replace_fenestration_vertices(payload.id, existing.vertices_m.clone())]
}
//#endregion 🔖️Inverse
