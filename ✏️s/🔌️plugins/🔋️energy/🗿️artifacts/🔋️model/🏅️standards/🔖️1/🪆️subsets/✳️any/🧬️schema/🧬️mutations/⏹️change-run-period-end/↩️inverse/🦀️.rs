//! ↩️ Inverse for `ChangeRunPeriodEndDay` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeRunPeriodEndDay, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let mut next = base.model.run_period;
    next.end_day = payload.new_end_day;
    if !(1..=31).contains(&payload.new_end_day) || base.model.run_period.end_day == payload.new_end_day || !next.is_interval() {
        return Vec::new();
    }
    vec![vocabulary::change_run_period_end_day(base.model.run_period.end_day)]
}
//#endregion 🔖️Inverse
