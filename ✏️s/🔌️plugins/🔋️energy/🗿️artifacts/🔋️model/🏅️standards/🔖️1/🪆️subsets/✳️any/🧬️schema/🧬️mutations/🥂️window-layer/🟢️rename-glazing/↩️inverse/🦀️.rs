//! ↩️ Inverse for `RenameGlazingMaterial` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RenameGlazingMaterial, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RenameGlazingMaterial>,
{
    Ok((|| {
    match base.model.glazing_materials.iter().find(|item| item.id == payload.id) {
        Some(item) if item.name != payload.new_name && !payload.new_name.trim().is_empty() => vec![vocabulary::rename_glazing_material(payload.id, item.name.clone())],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
