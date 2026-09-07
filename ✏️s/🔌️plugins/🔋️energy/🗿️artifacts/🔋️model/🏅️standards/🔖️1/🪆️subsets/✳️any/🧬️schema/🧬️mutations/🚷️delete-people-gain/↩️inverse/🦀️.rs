//! ↩️ Inverse for `DeletePeopleGain` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeletePeopleGain, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(index) = base.model.people.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.people[index];
    vec![vocabulary::create_people_gain(index as u32, existing.id, existing.zone_id, existing.schedule_id, existing.activity_schedule_id, existing.people_per_area, existing.sensible_fraction, existing.latent_fraction, existing.radiant_fraction)]
}
//#endregion 🔖️Inverse
