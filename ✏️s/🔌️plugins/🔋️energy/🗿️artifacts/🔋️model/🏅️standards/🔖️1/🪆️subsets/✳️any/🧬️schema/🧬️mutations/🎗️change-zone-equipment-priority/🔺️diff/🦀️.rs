//! 🔺️ Sparse diff builder for `ChangeZoneEquipmentPriority` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneEquipmentPriority, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zone_equipment.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone equipment {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_priority == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "An equipment priority must be at least one.".to_string(), [payload.id.0.to_string()]);
    }
    if existing.priority == payload.new_priority {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone equipment {} already has that priority.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.zone_equipment.iter_mut().find(|item| item.id == payload.id) {
        item.priority = payload.new_priority;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
