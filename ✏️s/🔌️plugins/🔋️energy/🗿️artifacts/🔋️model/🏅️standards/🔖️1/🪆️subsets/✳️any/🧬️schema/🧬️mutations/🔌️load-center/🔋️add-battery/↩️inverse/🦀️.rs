//! ↩️ Inverse for `AddElectricalLoadCenterBattery` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::AddElectricalLoadCenterBattery, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::AddElectricalLoadCenterBattery> + From<crate::RemoveElectricalLoadCenterBattery>,
{
    Ok((|| {
    match base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) {
        Some(item) if !item.battery_ids.contains(&payload.battery_id) && payload.index as usize <= item.battery_ids.len() && base.model.battery_storage.iter().any(|row| row.id == payload.battery_id) => {
            vec![vocabulary::remove_electrical_load_center_battery(payload.id, payload.battery_id)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
