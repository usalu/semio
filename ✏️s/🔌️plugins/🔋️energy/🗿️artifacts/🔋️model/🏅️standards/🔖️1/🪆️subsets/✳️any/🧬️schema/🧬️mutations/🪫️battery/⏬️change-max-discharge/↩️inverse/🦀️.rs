//! ↩️ Inverse for `ChangeBatteryMaxDischarge` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeBatteryMaxDischarge, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeBatteryMaxDischarge>,
{
    Ok((|| {
    match base.model.battery_storage.iter().find(|item| item.id == payload.id) {
        Some(item) if item.max_discharge_w != payload.new_max_discharge_w && !(!payload.new_max_discharge_w.is_finite() || payload.new_max_discharge_w <= 0.0) => vec![vocabulary::change_battery_max_discharge(payload.id, item.max_discharge_w)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
