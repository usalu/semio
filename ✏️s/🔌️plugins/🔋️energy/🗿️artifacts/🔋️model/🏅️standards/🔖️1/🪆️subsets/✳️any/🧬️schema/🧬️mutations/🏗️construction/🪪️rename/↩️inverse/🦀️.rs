//! ↩️ Inverse for `RenameConstruction` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RenameConstruction, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RenameConstruction>,
{
    Ok((|| {
    match base.model.constructions.iter().find(|item| item.id == payload.id) {
        Some(item) if item.name != payload.new_name && !payload.new_name.trim().is_empty() && !base.model.constructions.iter().any(|other| other.id != payload.id && other.name == payload.new_name) => {
            vec![vocabulary::rename_construction(payload.id, item.name.clone())]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
