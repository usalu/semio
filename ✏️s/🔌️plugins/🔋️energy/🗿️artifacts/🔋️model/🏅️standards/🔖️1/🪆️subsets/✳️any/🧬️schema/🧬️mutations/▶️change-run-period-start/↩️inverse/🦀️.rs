//! ↩️ Inverse for `ChangeRunPeriodStartDay` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeRunPeriodStartDay, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let mut next = base.model.run_period;
    next.start_day = payload.new_start_day;
    if !(1..=31).contains(&payload.new_start_day) || base.model.run_period.start_day == payload.new_start_day || !next.is_interval() {
        return Vec::new();
    }
    vec![vocabulary::change_run_period_start_day(base.model.run_period.start_day)]
}
//#endregion 🔖️Inverse
