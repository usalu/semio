//! 🔺️ Sparse diff builder for `CreateThermalEnclosure` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateThermalEnclosure, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.thermal_enclosures.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Thermal enclosure {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.thermal_enclosures.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} thermal_enclosures.", payload.index, base.model.thermal_enclosures.len()), [payload.id.0.to_string()]);
    }
    if let Some(missing) = payload.zone_ids.iter().find(|candidate| !base.model.zones.iter().any(|row| row.id == **candidate)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", missing.0), [missing.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.thermal_enclosures.insert(payload.index as usize, crate::model::ThermalEnclosure { id: payload.id, name: payload.name.clone(), zone_ids: payload.zone_ids.clone() });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
