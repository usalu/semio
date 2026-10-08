//! 🔺️ Sparse diff builder for `ChangeFaultStartSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FaultDefinitionPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFaultStartSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.faults.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fault {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.new_start_schedule_id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.new_start_schedule_id))
    {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.new_start_schedule_id.0), [payload.new_start_schedule_id.0.to_string()]);
    }
    if existing.start_schedule_id == payload.new_start_schedule_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fault {} already carries this schedule reference: {}.", payload.id.0, payload.new_start_schedule_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { faults: Rows::modifying(FaultDefinitionPatch { start_schedule_id: Some(payload.new_start_schedule_id), ..FaultDefinitionPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
