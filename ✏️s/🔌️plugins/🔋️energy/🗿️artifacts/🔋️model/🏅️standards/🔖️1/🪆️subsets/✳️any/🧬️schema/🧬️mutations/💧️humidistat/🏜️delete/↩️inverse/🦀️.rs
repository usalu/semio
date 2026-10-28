//! ↩️ Inverse for `DeleteHumidistat` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteHumidistat, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteHumidistat> + From<crate::CreateHumidistat>,
{
    Ok((|| {
    match base.model.humidistats.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((index, item)) => vec![vocabulary::create_humidistat(item.id, item.zone_id, item.humidifying_setpoint_schedule_id, item.dehumidifying_setpoint_schedule_id, item.humidifying_throttle_range, item.dehumidifying_throttle_range, Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
