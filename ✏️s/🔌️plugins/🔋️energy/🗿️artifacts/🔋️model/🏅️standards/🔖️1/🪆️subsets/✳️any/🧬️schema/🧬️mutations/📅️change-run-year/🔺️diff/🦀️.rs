//! 🔺️ Sparse diff builder for `ChangeRunYear` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, RunPeriodPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRunYear, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.run_period.year == payload.new_year {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The run period year is already {}.", payload.new_year));
    }
    let mut period = base.model.run_period;
    period.year = payload.new_year;
    if !period.is_interval() {
        let period = base.model.run_period;
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("A year of {} does not form a calendar interval with the run period {}-{} .. {}-{} of {}.", payload.new_year, period.start_month, period.start_day, period.end_month, period.end_day, period.year), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { run_period: RunPeriodPatch { year: Some(payload.new_year), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
