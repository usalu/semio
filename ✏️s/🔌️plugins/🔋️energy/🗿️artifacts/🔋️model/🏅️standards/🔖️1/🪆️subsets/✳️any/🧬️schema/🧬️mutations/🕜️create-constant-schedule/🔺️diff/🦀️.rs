//! 🔺️ Sparse diff builder for `CreateConstantSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ConstantSchedulePatch, ModelPatch, Rows, ScheduleSetPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateConstantSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id)
    {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Schedule {} is already defined.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.schedules.constants.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} constants schedules.", payload.index, base.model.schedules.constants.len()), [payload.id.0.to_string()]);
    }
    if !payload.value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Schedule {} needs a finite value, got {}.", payload.id.0, payload.value), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { schedules: ScheduleSetPatch { constants: Rows::inserting(payload.index as usize, crate::schedule::ConstantSchedule { id: payload.id, value: payload.value }), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
