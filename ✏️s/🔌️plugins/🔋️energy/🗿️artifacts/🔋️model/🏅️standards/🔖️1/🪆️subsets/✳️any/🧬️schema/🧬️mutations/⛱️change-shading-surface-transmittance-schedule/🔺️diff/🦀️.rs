//! 🔺️ Sparse diff builder for `ChangeShadingSurfaceTransmittanceSchedule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeShadingSurfaceTransmittanceSchedule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shading surface {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_transmittance_schedule_id.is_some_and(|schedule| !base.model.schedules.contains(schedule)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "The named transmittance schedule is not defined by this model.", [payload.id.0.to_string()]);
    }
    if existing.transmittance_schedule_id == payload.new_transmittance_schedule_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shading surface {} already has this transmittance schedule.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.shading_surfaces.iter_mut().find(|item| item.id == payload.id) {
        item.transmittance_schedule_id = payload.new_transmittance_schedule_id;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
