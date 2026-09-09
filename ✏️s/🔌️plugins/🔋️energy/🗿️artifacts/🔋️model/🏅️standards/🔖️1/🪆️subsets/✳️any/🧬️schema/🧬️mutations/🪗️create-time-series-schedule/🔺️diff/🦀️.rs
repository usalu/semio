//! 🔺️ Sparse diff builder for `CreateTimeSeriesSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateTimeSeriesSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.id)
        || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.id)
    {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Schedule {} is already defined.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.schedules.time_series.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} time_series schedules.", payload.index, base.model.schedules.time_series.len()), [payload.id.0.to_string()]);
    }
    if payload.values.is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule carries at least one value.", [payload.id.0.to_string()]);
    }
    if payload.values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", "Every time series value must be finite.", [payload.id.0.to_string()]);
    }
    if payload.timestep_seconds == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule needs a timestep of at least one second.", [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.schedules.time_series.insert(payload.index as usize, crate::schedule::TimeSeriesSchedule { id: payload.id, values: payload.values.clone(), timestep_seconds: payload.timestep_seconds });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
