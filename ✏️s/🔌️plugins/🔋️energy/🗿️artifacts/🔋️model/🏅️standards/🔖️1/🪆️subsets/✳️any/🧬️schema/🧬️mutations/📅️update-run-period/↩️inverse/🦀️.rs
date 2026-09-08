//! ↩️ Inverse for `UpdateRunPeriod` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::UpdateRunPeriod, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let run_period = crate::calendar::RunPeriod { start_month: payload.start_month, start_day: payload.start_day, end_month: payload.end_month, end_day: payload.end_day, year: payload.year };
    if base.model.run_period == run_period {
        return Vec::new();
    }
    let old = base.model.run_period;
    vec![vocabulary::update_run_period(old.start_month, old.start_day, old.end_month, old.end_day, old.year)]
}
//#endregion 🔖️Inverse
