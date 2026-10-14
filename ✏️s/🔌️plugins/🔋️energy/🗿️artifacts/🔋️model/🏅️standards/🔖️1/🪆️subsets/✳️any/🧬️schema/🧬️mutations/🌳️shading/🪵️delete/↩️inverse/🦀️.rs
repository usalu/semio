//! ↩️ Inverse for `DeleteShadingSurface` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteShadingSurface, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some((index, existing)) = base.model.shading_surfaces.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    vec![vocabulary::create_shading_surface(existing.id, existing.name.clone(), existing.vertices_m.clone(), existing.transmittance_schedule_id, Some(index as u32))]

    })())
}
//#endregion 🔖️Inverse
