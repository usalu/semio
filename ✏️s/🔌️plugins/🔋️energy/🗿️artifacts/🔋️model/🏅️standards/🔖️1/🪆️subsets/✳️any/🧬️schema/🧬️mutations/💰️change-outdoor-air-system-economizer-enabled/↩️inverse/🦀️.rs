//! ↩️ Inverse for `ChangeOutdoorAirSystemEconomizerEnabled` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeOutdoorAirSystemEconomizerEnabled, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.economizer_enabled != payload.new_economizer_enabled => vec![vocabulary::change_outdoor_air_system_economizer_enabled(payload.id, item.economizer_enabled)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
