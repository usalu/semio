//! ↩️ Inverse for `ChangeRunPeriodYear` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeRunPeriodYear, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let mut next = base.model.run_period;
    next.year = payload.new_year;
    if base.model.run_period.year == payload.new_year || !next.is_interval() {
        return Vec::new();
    }
    vec![vocabulary::change_run_period_year(base.model.run_period.year)]
}
//#endregion 🔖️Inverse
