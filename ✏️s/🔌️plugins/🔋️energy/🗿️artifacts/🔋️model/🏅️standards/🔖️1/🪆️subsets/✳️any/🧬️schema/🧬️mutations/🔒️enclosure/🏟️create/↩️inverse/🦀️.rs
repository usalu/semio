//! ↩️ Inverse for `CreateThermalEnclosure` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateThermalEnclosure, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateThermalEnclosure> + From<crate::DeleteThermalEnclosure>,
{
    Ok((|| {
    if base.model.thermal_enclosures.iter().any(|item| item.id == payload.id) || payload.index as usize > base.model.thermal_enclosures.len() || payload.zone_ids.iter().any(|candidate| !base.model.zones.iter().any(|row| row.id == *candidate)) {
        return Vec::new();
    }
    vec![vocabulary::delete_thermal_enclosure(payload.id)]

    })())
}
//#endregion 🔖️Inverse
