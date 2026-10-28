//! ↩️ Inverse for `ReplaceFenestrationVertices` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ReplaceFenestrationVertices, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ReplaceFenestrationVertices>,
{
    Ok((|| {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
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

    })())
}
//#endregion 🔖️Inverse
