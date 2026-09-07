//! ↩️ Inverse for `ChangeOutdoorAirSystemMinOaFlow` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeOutdoorAirSystemMinOaFlow, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.min_oa_flow_m3_s != payload.new_min_oa_flow_m3_s && !((!payload.new_min_oa_flow_m3_s.is_finite() || payload.new_min_oa_flow_m3_s < 0.0)) => vec![vocabulary::change_outdoor_air_system_min_oa_flow(payload.id, item.min_oa_flow_m3_s)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
