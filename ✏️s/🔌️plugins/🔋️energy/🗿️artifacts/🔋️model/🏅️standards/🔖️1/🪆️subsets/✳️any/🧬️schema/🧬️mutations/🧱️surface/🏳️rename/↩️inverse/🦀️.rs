//! ↩️ Inverse for `RenameSurface` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RenameSurface, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RenameSurface>,
{
    Ok((|| {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.new_name.trim().is_empty() || base.model.surfaces.iter().any(|item| item.id != payload.id && item.name == payload.new_name) || existing.name == payload.new_name {
        return Vec::new();
    }
    vec![vocabulary::rename_surface(payload.id, existing.name.clone())]

    })())
}
//#endregion 🔖️Inverse
