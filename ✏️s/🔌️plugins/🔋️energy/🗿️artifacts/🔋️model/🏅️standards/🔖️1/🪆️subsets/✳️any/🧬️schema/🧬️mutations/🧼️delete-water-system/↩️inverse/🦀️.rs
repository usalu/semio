//! ↩️ Inverse for `DeleteWaterSystem` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteWaterSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.water_systems.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.water_systems[index];
    vec![vocabulary::create_water_system(index as u32, existing.id, existing.fixture_count, existing.peak_flow_l_s, existing.schedule_id)]
}
//#endregion 🔖️Inverse
