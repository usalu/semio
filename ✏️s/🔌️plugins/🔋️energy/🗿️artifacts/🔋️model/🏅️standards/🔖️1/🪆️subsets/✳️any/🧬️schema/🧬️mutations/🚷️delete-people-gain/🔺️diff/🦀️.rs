//! 🔺️ Sparse diff builder for `DeletePeopleGain` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, PeopleGainPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeletePeopleGain, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.people.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("People Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { people: Rows::removing(&base.model.people, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
