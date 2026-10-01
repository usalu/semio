//! ↩️ Inverse for `ChangeLightingGainReturnAirFraction` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeLightingGainReturnAirFraction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.lighting.iter().find(|item| item.id == payload.id) {
        Some(item) if item.return_air_fraction != payload.new_return_air_fraction && !(!(0.0..=1.0).contains(&payload.new_return_air_fraction)) => vec![vocabulary::change_lighting_gain_return_air_fraction(payload.id, item.return_air_fraction)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
