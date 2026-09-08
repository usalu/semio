//! 🔺️ Sparse diff builder for `RemoveThermalEnclosureZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveThermalEnclosureZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermal_enclosures.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal enclosure {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.zone_ids.contains(&payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} is not a member of Thermal enclosure {}.", payload.zone_id.0, payload.id.0), [payload.zone_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.thermal_enclosures.iter_mut().find(|item| item.id == payload.id) {
        item.zone_ids.retain(|candidate| *candidate != payload.zone_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
