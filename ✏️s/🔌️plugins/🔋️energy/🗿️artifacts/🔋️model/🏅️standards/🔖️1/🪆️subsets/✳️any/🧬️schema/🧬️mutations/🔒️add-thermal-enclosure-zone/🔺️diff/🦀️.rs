//! 🔺️ Sparse diff builder for `AddThermalEnclosureZone` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddThermalEnclosureZone, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermal_enclosures.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal enclosure {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.zones.iter().any(|row| row.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if payload.index as usize > existing.zone_ids.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of Thermal enclosure {}'s {} members.", payload.index, payload.id.0, existing.zone_ids.len()), [payload.id.0.to_string()]);
    }
    if existing.zone_ids.contains(&payload.zone_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone {} already belongs to Thermal enclosure {}.", payload.zone_id.0, payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.thermal_enclosures.iter_mut().find(|item| item.id == payload.id) {
        item.zone_ids.insert(payload.index as usize, payload.zone_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
