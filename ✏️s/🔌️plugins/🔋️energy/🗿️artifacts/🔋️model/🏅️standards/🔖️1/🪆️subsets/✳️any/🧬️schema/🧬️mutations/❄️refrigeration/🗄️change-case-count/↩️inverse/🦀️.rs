//! ↩️ Inverse for `ChangeRefrigerationSystemCaseCount` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeRefrigerationSystemCaseCount, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeRefrigerationSystemCaseCount>,
{
    Ok((|| {
    match base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.case_count != payload.new_case_count && payload.new_case_count != 0 => vec![vocabulary::change_refrigeration_system_case_count(payload.id, item.case_count)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
