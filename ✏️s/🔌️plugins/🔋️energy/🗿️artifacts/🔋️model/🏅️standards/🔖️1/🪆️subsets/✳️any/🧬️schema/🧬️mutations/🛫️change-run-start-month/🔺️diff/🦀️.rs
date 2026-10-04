//! 🔺️ Sparse diff builder for `ChangeRunStartMonth` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRunStartMonth, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(1..=12).contains(&payload.new_start_month) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A run period start month of {} is not admissible.", payload.new_start_month), Vec::<String>::new());
    }
    if base.model.run_period.start_month == payload.new_start_month {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The run period start month is already {}.", payload.new_start_month));
    }
    let mut model = base.model.clone();
    model.run_period.start_month = payload.new_start_month;
    if !model.run_period.is_interval() {
        let period = base.model.run_period;
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("A start month of {} does not form a calendar interval with the run period {}-{} .. {}-{} of {}.", payload.new_start_month, period.start_month, period.start_day, period.end_month, period.end_day, period.year), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
