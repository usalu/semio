//! ↩️ Inverse for `ChangeRunStartMonth` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeRunStartMonth, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeRunStartMonth>,
{
    Ok((|| {
    let mut next = base.model.run_period;
    next.start_month = payload.new_start_month;
    if !(1..=12).contains(&payload.new_start_month) || base.model.run_period.start_month == payload.new_start_month || !next.is_interval() {
        return Vec::new();
    }
    vec![vocabulary::change_run_start_month(base.model.run_period.start_month)]

    })())
}
//#endregion 🔖️Inverse
