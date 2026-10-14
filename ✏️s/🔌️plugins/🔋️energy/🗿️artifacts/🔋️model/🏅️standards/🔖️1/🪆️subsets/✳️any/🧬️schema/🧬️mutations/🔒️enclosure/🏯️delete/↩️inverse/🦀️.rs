//! ↩️ Inverse for `DeleteThermalEnclosure` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteThermalEnclosure, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteThermalEnclosure> + From<crate::CreateThermalEnclosure>,
{
    Ok((|| {
    let Some(index) = base.model.thermal_enclosures.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.thermal_enclosures[index];
    vec![vocabulary::create_thermal_enclosure(index as u32, existing.id, existing.name.clone(), existing.zone_ids.clone())]

    })())
}
//#endregion 🔖️Inverse
