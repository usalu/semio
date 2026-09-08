//! 🔺️ Sparse diff builder for `ReplaceTimeSeriesScheduleValues` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceTimeSeriesScheduleValues, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.time_series.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Time series schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_values.is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule carries at least one value.", [payload.id.0.to_string()]);
    }
    if payload.new_values.iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::error("mutation.invariant", "Every time series value must be finite.", [payload.id.0.to_string()]);
    }
    if existing.values == payload.new_values {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Time series schedule {} already carries this values: {:?}.", payload.id.0, payload.new_values));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.time_series.iter_mut().find(|item| item.id == payload.id) {
        item.values = payload.new_values.clone();
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
