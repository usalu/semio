//! 🔺️ Sparse diff builder for `ChangeZoneEquipmentCoolingCapacity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneEquipmentCoolingCapacity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zone_equipment.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone equipment {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_cooling_capacity_w.is_finite() || payload.new_cooling_capacity_w < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A cooling capacity must be a non-negative finite number, got {}.", payload.new_cooling_capacity_w), [payload.id.0.to_string()]);
    }
    if existing.cooling_capacity_w == payload.new_cooling_capacity_w {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Zone equipment {} already has that cooling capacity.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.zone_equipment.iter_mut().find(|item| item.id == payload.id) {
        item.cooling_capacity_w = payload.new_cooling_capacity_w;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
