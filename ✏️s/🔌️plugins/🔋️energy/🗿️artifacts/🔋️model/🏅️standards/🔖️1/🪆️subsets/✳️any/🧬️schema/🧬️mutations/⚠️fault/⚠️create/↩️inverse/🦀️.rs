//! ↩️ Inverse for `CreateFault` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateFault, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateFault> + From<crate::DeleteFault>,
{
    Ok((|| {
    if base.model.faults.iter().any(|item| item.id == payload.id)
        || payload.index as usize > base.model.faults.len()
        || !base.model.ideal_loads.iter().any(|row| row.id == payload.target_equipment_id)
        || !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.start_schedule_id)
            || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.start_schedule_id)
            || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.start_schedule_id)
            || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.start_schedule_id)
            || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.start_schedule_id))
    {
        return Vec::new();
    }
    vec![vocabulary::delete_fault(payload.id)]

    })())
}
//#endregion 🔖️Inverse
