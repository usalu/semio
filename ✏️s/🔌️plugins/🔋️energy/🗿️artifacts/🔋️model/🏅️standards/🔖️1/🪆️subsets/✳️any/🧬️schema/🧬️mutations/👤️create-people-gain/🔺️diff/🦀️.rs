//! 🔺️ Sparse diff builder for `CreatePeopleGain` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreatePeopleGain, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.people.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("People Gain {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.people.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} people.", payload.index, base.model.people.len()), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.schedule_id)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.schedule_id.0), [payload.schedule_id.0.to_string()]);
    }
    if !(base.model.schedules.constants.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.daily.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.weekly.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.annual.iter().any(|schedule| schedule.id == payload.activity_schedule_id) || base.model.schedules.time_series.iter().any(|schedule| schedule.id == payload.activity_schedule_id)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Schedule {} does not exist.", payload.activity_schedule_id.0), [payload.activity_schedule_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.people.insert(payload.index as usize, crate::model::PeopleGain { id: payload.id, zone_id: payload.zone_id, schedule_id: payload.schedule_id, activity_schedule_id: payload.activity_schedule_id, people_per_area: payload.people_per_area, sensible_fraction: payload.sensible_fraction, latent_fraction: payload.latent_fraction, radiant_fraction: payload.radiant_fraction });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
