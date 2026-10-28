//! ↩️ Inverse for `CreateConstantSchedule` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateConstantSchedule, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateConstantSchedule> + From<crate::DeleteConstantSchedule>,
{
    Ok((|| {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id)
        || payload.index as usize > base.model.schedules.constants.len()
        || !payload.value.is_finite()
    {
        return Vec::new();
    }
    vec![vocabulary::delete_constant_schedule(payload.id)]

    })())
}
//#endregion 🔖️Inverse
