//! 🔺️ Sparse diff builder for `UpdateRunPeriod` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateRunPeriod, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let run_period = crate::calendar::RunPeriod { start_month: payload.start_month, start_day: payload.start_day, end_month: payload.end_month, end_day: payload.end_day, year: payload.year };
    if !(1..=12).contains(&run_period.start_month) || !(1..=12).contains(&run_period.end_month) || !(1..=31).contains(&run_period.start_day) || !(1..=31).contains(&run_period.end_day) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Run period {}-{} .. {}-{} is not a calendar interval.", run_period.start_month, run_period.start_day, run_period.end_month, run_period.end_day), Vec::<String>::new());
    }
    if base.model.run_period == run_period {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The run period already has this value.");
    }
    let mut model = base.model.clone();
    model.run_period = run_period;
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
