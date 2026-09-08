//! ↩️ Inverse for `ChangeOutdoorAirSystemAirLoop` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeOutdoorAirSystemAirLoop, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.air_loop_id != payload.new_air_loop_id && !(!base.model.air_loops.iter().any(|item| item.id == payload.new_air_loop_id)) => vec![vocabulary::change_outdoor_air_system_air_loop(payload.id, item.air_loop_id)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
