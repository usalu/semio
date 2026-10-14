//! ↩️ Inverse for `ChangeSurfaceZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSurfaceZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSurfaceZone>,
{
    Ok((|| {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !base.model.zones.iter().any(|item| item.id == payload.new_zone_id) || existing.zone_id == payload.new_zone_id {
        return Vec::new();
    }
    vec![vocabulary::change_surface_zone(payload.id, existing.zone_id)]

    })())
}
//#endregion 🔖️Inverse
