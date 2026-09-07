//! ↩️ Inverse for `RenameShadingSurface` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::RenameShadingSurface, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.new_name.trim().is_empty() || base.model.shading_surfaces.iter().any(|item| item.id != payload.id && item.name == payload.new_name) || existing.name == payload.new_name {
        return Vec::new();
    }
    vec![vocabulary::rename_shading_surface(payload.id, existing.name.clone())]
}
//#endregion 🔖️Inverse
