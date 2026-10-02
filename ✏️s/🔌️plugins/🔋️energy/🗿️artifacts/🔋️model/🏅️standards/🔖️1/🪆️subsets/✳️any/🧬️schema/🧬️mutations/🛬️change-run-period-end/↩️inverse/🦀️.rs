//! ↩️ Inverse for `ChangeRunPeriodEndMonth` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeRunPeriodEndMonth, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let mut next = base.model.run_period;
    next.end_month = payload.new_end_month;
    if !(1..=12).contains(&payload.new_end_month) || base.model.run_period.end_month == payload.new_end_month || !next.is_interval() {
        return Vec::new();
    }
    vec![vocabulary::change_run_period_end_month(base.model.run_period.end_month)]
}
//#endregion 🔖️Inverse
