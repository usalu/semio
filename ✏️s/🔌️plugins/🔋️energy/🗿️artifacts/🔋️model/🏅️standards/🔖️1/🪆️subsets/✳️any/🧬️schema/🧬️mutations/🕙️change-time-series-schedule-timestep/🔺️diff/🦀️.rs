//! 🔺️ Sparse diff builder for `ChangeTimeSeriesScheduleTimestep` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeTimeSeriesScheduleTimestep, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.time_series.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Time series schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_timestep_seconds == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "A time series schedule needs a timestep of at least one second.", [payload.id.0.to_string()]);
    }
    if existing.timestep_seconds == payload.new_timestep_seconds {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Time series schedule {} already carries this timestep_seconds: {}.", payload.id.0, payload.new_timestep_seconds));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.time_series.iter_mut().find(|item| item.id == payload.id) {
        item.timestep_seconds = payload.new_timestep_seconds;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
