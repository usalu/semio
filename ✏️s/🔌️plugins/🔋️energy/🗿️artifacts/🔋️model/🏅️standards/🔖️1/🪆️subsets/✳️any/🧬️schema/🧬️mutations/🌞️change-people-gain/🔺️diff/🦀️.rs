//! 🔺️ Sparse diff builder for `ChangePeopleGainSensibleFraction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, PeopleGainPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePeopleGainSensibleFraction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.people.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("People Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_sensible_fraction) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("People Gain {}: sensible fraction must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_sensible_fraction), [payload.id.0.to_string()]);
    }
    if existing.sensible_fraction == payload.new_sensible_fraction {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("People Gain {} already carries this sensible fraction: {}.", payload.id.0, payload.new_sensible_fraction));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { people: Rows::modifying(PeopleGainPatch { sensible_fraction: Some(payload.new_sensible_fraction), ..PeopleGainPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
