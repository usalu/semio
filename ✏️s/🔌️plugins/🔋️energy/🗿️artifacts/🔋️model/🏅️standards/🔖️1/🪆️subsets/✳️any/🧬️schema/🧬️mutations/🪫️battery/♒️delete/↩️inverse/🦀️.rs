//! ↩️ Inverse for `DeleteBattery` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteBattery, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteBattery> + From<crate::CreateBattery>,
{
    Ok((|| {
    let Some(index) = base.model.battery_storage.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.electrical_load_centers.iter().any(|centre| centre.battery_ids.contains(&payload.id)) {
        return Vec::new();
    }
    let existing = &base.model.battery_storage[index];
    vec![vocabulary::create_battery(index as u32, existing.id, existing.capacity_kwh, existing.max_charge_w, existing.max_discharge_w, existing.round_trip_efficiency)]

    })())
}
//#endregion 🔖️Inverse
