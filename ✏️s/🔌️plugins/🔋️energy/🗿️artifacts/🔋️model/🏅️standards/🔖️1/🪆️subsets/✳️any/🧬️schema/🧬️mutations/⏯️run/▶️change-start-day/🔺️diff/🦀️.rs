//! 🔺️ Sparse diff builder for `ChangeRunStartDay` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, RunPeriodPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRunStartDay, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(1..=31).contains(&payload.new_start_day) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A run period start day of {} is not admissible.", payload.new_start_day), Vec::<String>::new());
    }
    if base.model.run_period.start_day == payload.new_start_day {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The run period start day is already {}.", payload.new_start_day));
    }
    let mut period = base.model.run_period;
    period.start_day = payload.new_start_day;
    if !period.is_interval() {
        let period = base.model.run_period;
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("A start day of {} does not form a calendar interval with the run period {}-{} .. {}-{} of {}.", payload.new_start_day, period.start_month, period.start_day, period.end_month, period.end_day, period.year), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { run_period: RunPeriodPatch { start_day: Some(payload.new_start_day), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
