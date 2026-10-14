//! ↩️ Inverse for `CreateShwSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateShwSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateShwSystem> + From<crate::DeleteShwSystem>,
{
    Ok((|| {
    if base.model.shw_systems.iter().any(|item| item.id == payload.id)
        || payload.index as usize > base.model.shw_systems.len()
        || !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id)
            || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id))
    {
        return Vec::new();
    }
    vec![vocabulary::delete_shw_system(payload.id)]

    })())
}
//#endregion 🔖️Inverse
