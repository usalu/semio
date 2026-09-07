//! 🔺️ Sparse diff builder for `RemoveElectricalLoadCenterPv` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveElectricalLoadCenterPv, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Electrical load center {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.pv_ids.contains(&payload.pv_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} is not a member of Electrical load center {}.", payload.pv_id.0, payload.id.0), [payload.pv_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.electrical_load_centers.iter_mut().find(|item| item.id == payload.id) {
        item.pv_ids.retain(|candidate| *candidate != payload.pv_id);
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
