//! ↩️ Inverse for `ChangeLightingGainVisibleFraction` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeLightingGainVisibleFraction, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    match base.model.lighting.iter().find(|item| item.id == payload.id) {
        Some(item) if item.visible_fraction != payload.new_visible_fraction && !(!(0.0..=1.0).contains(&payload.new_visible_fraction)) => vec![vocabulary::change_lighting_gain_visible_fraction(payload.id, item.visible_fraction)],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
