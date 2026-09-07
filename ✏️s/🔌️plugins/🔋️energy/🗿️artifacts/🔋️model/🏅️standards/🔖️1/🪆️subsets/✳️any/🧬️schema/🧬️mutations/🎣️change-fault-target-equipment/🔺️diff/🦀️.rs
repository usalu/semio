//! 🔺️ Sparse diff builder for `ChangeFaultTargetEquipment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFaultTargetEquipment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.faults.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fault {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.ideal_loads.iter().any(|row| row.id == payload.new_target_equipment_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.new_target_equipment_id.0), [payload.new_target_equipment_id.0.to_string()]);
    }
    if existing.target_equipment_id == payload.new_target_equipment_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fault {} already carries this target_equipment_id: {}.", payload.id.0, payload.new_target_equipment_id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.faults.iter_mut().find(|item| item.id == payload.id) {
        item.target_equipment_id = payload.new_target_equipment_id;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
