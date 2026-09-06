//! ↩️ Inverse for `ChangeZoneMultiplier` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeZoneMultiplier, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.multiplier != payload.new_multiplier && payload.new_multiplier != 0 => vec![vocabulary::change_zone_multiplier(payload.id, zone.multiplier)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
