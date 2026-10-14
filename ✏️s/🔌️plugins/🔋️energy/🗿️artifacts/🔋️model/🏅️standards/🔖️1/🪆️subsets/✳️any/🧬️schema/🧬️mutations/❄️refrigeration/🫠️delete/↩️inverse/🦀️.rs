//! ↩️ Inverse for `DeleteRefrigerationSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteRefrigerationSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteRefrigerationSystem> + From<crate::CreateRefrigerationSystem>,
{
    Ok((|| {
    let Some(index) = base.model.refrigeration_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.refrigeration_systems[index];
    vec![vocabulary::create_refrigeration_system(index as u32, existing.id, existing.case_count, existing.design_load_w, existing.defrost_schedule_id)]

    })())
}
//#endregion 🔖️Inverse
