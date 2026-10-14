//! ↩️ Inverse for `CreateConstruction` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateConstruction, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateConstruction> + From<crate::DeleteConstruction>,
{
    Ok((|| {
    if base.model.constructions.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.constructions.len() || payload.layer_material_ids.iter().any(|id| !base.model.materials.iter().any(|material| material.id == *id)) {
        return Vec::new();
    }
    vec![vocabulary::delete_construction(payload.id)]

    })())
}
//#endregion 🔖️Inverse
