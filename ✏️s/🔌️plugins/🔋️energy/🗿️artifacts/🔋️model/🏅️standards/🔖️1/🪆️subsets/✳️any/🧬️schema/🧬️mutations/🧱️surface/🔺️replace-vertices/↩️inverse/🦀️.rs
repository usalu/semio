//! ↩️ Inverse for `ReplaceSurfaceVertices` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ReplaceSurfaceVertices, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ReplaceSurfaceVertices>,
{
    Ok((|| {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.new_vertices_m.len() < 3 || existing.vertices_m == payload.new_vertices_m {
        return Vec::new();
    }
    vec![vocabulary::replace_surface_vertices(payload.id, existing.vertices_m.clone())]

    })())
}
//#endregion 🔖️Inverse
