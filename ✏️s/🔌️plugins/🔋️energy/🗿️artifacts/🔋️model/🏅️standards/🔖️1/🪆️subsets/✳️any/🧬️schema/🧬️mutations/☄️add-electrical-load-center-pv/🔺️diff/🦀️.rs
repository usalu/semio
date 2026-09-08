//! 🔺️ Sparse diff builder for `AddElectricalLoadCenterPv` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddElectricalLoadCenterPv, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Electrical load center {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.pv_systems.iter().any(|row| row.id == payload.pv_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", payload.pv_id.0), [payload.pv_id.0.to_string()]);
    }
    if payload.index as usize > existing.pv_ids.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of Electrical load center {}'s {} members.", payload.index, payload.id.0, existing.pv_ids.len()), [payload.id.0.to_string()]);
    }
    if existing.pv_ids.contains(&payload.pv_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("PV system {} already belongs to Electrical load center {}.", payload.pv_id.0, payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.electrical_load_centers.iter_mut().find(|item| item.id == payload.id) {
        item.pv_ids.insert(payload.index as usize, payload.pv_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
