//! ↩️ Inverse for `DeleteShadingSurface` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteShadingSurface, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if false {
        return Vec::new();
    }
    vec![vocabulary::create_shading_surface(existing.id, existing.name.clone(), existing.vertices_m.clone(), existing.transmittance_schedule_id)]
}
//#endregion 🔖️Inverse
