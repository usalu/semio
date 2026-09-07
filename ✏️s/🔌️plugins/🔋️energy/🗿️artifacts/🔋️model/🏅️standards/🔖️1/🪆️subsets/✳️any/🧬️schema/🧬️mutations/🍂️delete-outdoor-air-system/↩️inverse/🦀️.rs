//! ↩️ Inverse for `DeleteOutdoorAirSystem` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteOutdoorAirSystem, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) {
        Some(item) => vec![vocabulary::create_outdoor_air_system(item.id, item.air_loop_id, item.min_oa_flow_m3_s, item.economizer_enabled)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
