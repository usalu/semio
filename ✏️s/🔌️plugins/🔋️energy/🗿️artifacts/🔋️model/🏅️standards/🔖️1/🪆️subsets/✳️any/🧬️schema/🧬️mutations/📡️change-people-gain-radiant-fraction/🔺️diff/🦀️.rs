//! 🔺️ Sparse diff builder for `ChangePeopleGainRadiantFraction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePeopleGainRadiantFraction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.people.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("People Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_radiant_fraction) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("People Gain {}: radiant fraction must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_radiant_fraction), [payload.id.0.to_string()]);
    }
    if existing.radiant_fraction == payload.new_radiant_fraction {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("People Gain {} already carries this radiant fraction: {}.", payload.id.0, payload.new_radiant_fraction));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.people.iter_mut().find(|item| item.id == payload.id) {
        item.radiant_fraction = payload.new_radiant_fraction;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
