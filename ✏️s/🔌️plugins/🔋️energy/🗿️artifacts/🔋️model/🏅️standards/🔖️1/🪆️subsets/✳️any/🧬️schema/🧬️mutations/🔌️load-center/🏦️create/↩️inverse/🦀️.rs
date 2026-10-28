//! ↩️ Inverse for `CreateElectricalLoadCenter` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateElectricalLoadCenter, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateElectricalLoadCenter> + From<crate::DeleteElectricalLoadCenter>,
{
    Ok((|| {
    if base.model.electrical_load_centers.iter().any(|item| item.id == payload.id)
        || payload.index as usize > base.model.electrical_load_centers.len()
        || payload.pv_ids.iter().any(|candidate| !base.model.pv_systems.iter().any(|row| row.id == *candidate))
        || payload.battery_ids.iter().any(|candidate| !base.model.battery_storage.iter().any(|row| row.id == *candidate))
    {
        return Vec::new();
    }
    vec![vocabulary::delete_electrical_load_center(payload.id)]

    })())
}
//#endregion 🔖️Inverse
