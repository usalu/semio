//! 🔺️ Sparse diff builder for `ChangeInfiltrationZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.new_zone_id.0), [payload.new_zone_id.0.to_string()]);
    }
    if existing.zone_id == payload.new_zone_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this zone reference: {}.", payload.id.0, payload.new_zone_id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.zone_id = payload.new_zone_id;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
