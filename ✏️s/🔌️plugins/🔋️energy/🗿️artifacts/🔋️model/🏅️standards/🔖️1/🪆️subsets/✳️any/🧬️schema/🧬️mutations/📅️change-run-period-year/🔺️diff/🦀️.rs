//! 🔺️ Sparse diff builder for `ChangeRunPeriodYear` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRunPeriodYear, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.run_period.year == payload.new_year {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The run period year is already {}.", payload.new_year));
    }
    let mut model = base.model.clone();
    model.run_period.year = payload.new_year;
    if !model.run_period.is_interval() {
        let period = base.model.run_period;
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("A year of {} does not form a calendar interval with the run period {}-{} .. {}-{} of {}.", payload.new_year, period.start_month, period.start_day, period.end_month, period.end_day, period.year), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
