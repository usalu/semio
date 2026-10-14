//! ↩️ Inverse for `CreateShadingSurface` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateShadingSurface, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateShadingSurface> + From<crate::DeleteShadingSurface>,
{
    Ok((|| {
    if base.model.shading_surfaces.iter().any(|item| item.id == payload.id)
        || payload.name.trim().is_empty()
        || payload.vertices_m.len() < 3
        || payload.transmittance_schedule_id.is_some_and(|schedule| !base.model.schedules.contains(schedule))
        || false
    {
        return Vec::new();
    }
    if payload.index.is_some_and(|index| index as usize > base.model.shading_surfaces.len()) {
        return Vec::new();
    }
    vec![vocabulary::delete_shading_surface(payload.id)]

    })())
}
//#endregion 🔖️Inverse
