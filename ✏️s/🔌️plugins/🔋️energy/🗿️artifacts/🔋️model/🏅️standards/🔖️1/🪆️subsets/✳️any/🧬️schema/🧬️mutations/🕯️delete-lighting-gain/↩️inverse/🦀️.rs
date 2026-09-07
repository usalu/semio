//! ↩️ Inverse for `DeleteLightingGain` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteLightingGain, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.lighting.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.lighting[index];
    vec![vocabulary::create_lighting_gain(index as u32, existing.id, existing.zone_id, existing.schedule_id, existing.watts_per_area, existing.radiant_fraction, existing.visible_fraction, existing.return_air_fraction)]
}
//#endregion 🔖️Inverse
