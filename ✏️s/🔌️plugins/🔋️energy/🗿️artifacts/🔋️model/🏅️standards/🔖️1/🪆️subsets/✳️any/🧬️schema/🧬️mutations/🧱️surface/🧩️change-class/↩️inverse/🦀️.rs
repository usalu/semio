//! ↩️ Inverse for `ChangeSurfaceClass` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSurfaceClass, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSurfaceClass>,
{
    Ok((|| {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if existing.class == payload.new_class {
        return Vec::new();
    }
    vec![vocabulary::change_surface_class(payload.id, existing.class)]

    })())
}
//#endregion 🔖️Inverse
