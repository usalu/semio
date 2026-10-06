//! 🔺️ Sparse diff builder for `ChangeRunEndDay` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRunEndDay, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(1..=31).contains(&payload.new_end_day) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A run period end day of {} is not admissible.", payload.new_end_day), Vec::<String>::new());
    }
    if base.model.run_period.end_day == payload.new_end_day {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The run period end day is already {}.", payload.new_end_day));
    }
    let mut model = base.model.clone();
    model.run_period.end_day = payload.new_end_day;
    if !model.run_period.is_interval() {
        let period = base.model.run_period;
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("A end day of {} does not form a calendar interval with the run period {}-{} .. {}-{} of {}.", payload.new_end_day, period.start_month, period.start_day, period.end_month, period.end_day, period.year), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(crate::standards::v1::subsets::any::schema::diff::diff_from_model(model))
}
//#endregion 🔖️Diff
