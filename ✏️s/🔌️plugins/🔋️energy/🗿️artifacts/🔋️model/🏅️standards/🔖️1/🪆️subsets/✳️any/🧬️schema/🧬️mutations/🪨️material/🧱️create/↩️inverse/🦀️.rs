//! ↩️ Inverse for `CreateMaterial` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateMaterial, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateMaterial> + From<crate::DeleteMaterial>,
{
    Ok((|| {
    if base.model.materials.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.materials.len() {
        return Vec::new();
    }
    vec![vocabulary::delete_material(payload.id)]

    })())
}
//#endregion 🔖️Inverse
