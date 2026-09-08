//! 🔺️ Sparse diff builder for `CreateConstantSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateConstantSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Schedule {} is already defined.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.schedules.constants.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} constants schedules.", payload.index, base.model.schedules.constants.len()), [payload.id.0.to_string()]);
    }
    if !payload.value.is_finite() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Schedule {} needs a finite value, got {}.", payload.id.0, payload.value), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.schedules.constants.insert(payload.index as usize, crate::schedule::ConstantSchedule { id: payload.id, value: payload.value });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
