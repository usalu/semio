//! 🔺️ Sparse diff builder for `ChangePeopleGainPeoplePerArea` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePeopleGainPeoplePerArea, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.people.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("People Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_people_per_area.is_finite() || payload.new_people_per_area < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("People Gain {}: occupant density (people/m²) must be a finite non-negative value, got {}.", payload.id.0, payload.new_people_per_area), [payload.id.0.to_string()]);
    }
    if existing.people_per_area == payload.new_people_per_area {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("People Gain {} already carries this occupant density (people/m²): {}.", payload.id.0, payload.new_people_per_area));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.people.iter_mut().find(|item| item.id == payload.id) {
        item.people_per_area = payload.new_people_per_area;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
