//! 🔺️ Sparse diff builder for `ChangeFaultType` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFaultType, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.faults.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fault {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.fault_type == payload.new_fault_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fault {} already carries this fault_type: {:?}.", payload.id.0, payload.new_fault_type));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.faults.iter_mut().find(|item| item.id == payload.id) {
        item.fault_type = payload.new_fault_type;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
