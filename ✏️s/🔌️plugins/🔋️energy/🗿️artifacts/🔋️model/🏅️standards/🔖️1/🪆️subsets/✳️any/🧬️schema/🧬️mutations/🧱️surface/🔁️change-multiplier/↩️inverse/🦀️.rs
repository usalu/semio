//! ↩️ Inverse for `ChangeSurfaceMultiplier` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSurfaceMultiplier, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSurfaceMultiplier>,
{
    Ok((|| {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.new_multiplier == 0 || existing.multiplier == payload.new_multiplier {
        return Vec::new();
    }
    vec![vocabulary::change_surface_multiplier(payload.id, existing.multiplier)]

    })())
}
//#endregion 🔖️Inverse
