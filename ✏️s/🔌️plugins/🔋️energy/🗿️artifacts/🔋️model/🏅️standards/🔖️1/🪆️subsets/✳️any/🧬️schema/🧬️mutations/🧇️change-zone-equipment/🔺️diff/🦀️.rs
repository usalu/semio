//! 🔺️ Sparse diff builder for `ChangeZoneEquipmentHeatingCapacity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ZoneEquipmentAssignmentPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneEquipmentHeatingCapacity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zone_equipment.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone equipment {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_heating_capacity_w.is_finite() || payload.new_heating_capacity_w < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A heating capacity must be a non-negative finite number, got {}.", payload.new_heating_capacity_w), [payload.id.0.to_string()]);
    }
    if existing.heating_capacity_w == payload.new_heating_capacity_w {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Zone equipment {} already has that heating capacity.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { zone_equipment: Rows::modifying(ZoneEquipmentAssignmentPatch { heating_capacity_w: Some(payload.new_heating_capacity_w), ..ZoneEquipmentAssignmentPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
