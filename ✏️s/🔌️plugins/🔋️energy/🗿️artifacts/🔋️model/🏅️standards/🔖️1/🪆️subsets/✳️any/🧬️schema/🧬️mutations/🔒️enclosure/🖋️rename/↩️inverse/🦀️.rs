//! ↩️ Inverse for `RenameThermalEnclosure` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RenameThermalEnclosure, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RenameThermalEnclosure>,
{
    Ok((|| {
    match base.model.thermal_enclosures.iter().find(|item| item.id == payload.id) {
        Some(item) if item.name != payload.new_name && !payload.new_name.trim().is_empty() && !base.model.thermal_enclosures.iter().any(|other| other.id != payload.id && other.name == payload.new_name) => {
            vec![vocabulary::rename_thermal_enclosure(payload.id, item.name.clone())]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
