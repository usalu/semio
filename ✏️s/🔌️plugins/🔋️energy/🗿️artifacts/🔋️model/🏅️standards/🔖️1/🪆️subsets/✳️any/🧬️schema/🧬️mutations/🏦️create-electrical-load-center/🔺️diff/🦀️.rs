//! 🔺️ Sparse diff builder for `CreateElectricalLoadCenter` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateElectricalLoadCenter, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.electrical_load_centers.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Electrical load center {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.electrical_load_centers.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} electrical_load_centers.", payload.index, base.model.electrical_load_centers.len()), [payload.id.0.to_string()]);
    }
    if let Some(missing) = payload.pv_ids.iter().find(|candidate| !base.model.pv_systems.iter().any(|row| row.id == **candidate)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", missing.0), [missing.0.to_string()]);
    }
    if let Some(missing) = payload.battery_ids.iter().find(|candidate| !base.model.battery_storage.iter().any(|row| row.id == **candidate)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} does not exist.", missing.0), [missing.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.electrical_load_centers.insert(payload.index as usize, crate::model::ElectricalLoadCenter { id: payload.id, name: payload.name.clone(), generator_ids: payload.generator_ids.clone(), pv_ids: payload.pv_ids.clone(), battery_ids: payload.battery_ids.clone() });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
