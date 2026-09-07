//! 🔺️ Sparse diff builder for `CreateZoneEquipment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateZoneEquipment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.zone_equipment.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Zone equipment {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    if payload.priority == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "An equipment priority must be at least one.".to_string(), [payload.id.0.to_string()]);
    }
    if !payload.heating_capacity_w.is_finite() || payload.heating_capacity_w < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A heating capacity must be a non-negative finite number, got {}.", payload.heating_capacity_w), [payload.id.0.to_string()]);
    }
    if !payload.cooling_capacity_w.is_finite() || payload.cooling_capacity_w < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A cooling capacity must be a non-negative finite number, got {}.", payload.cooling_capacity_w), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.zone_equipment.push(crate::model::ZoneEquipmentAssignment { id: payload.id, zone_id: payload.zone_id, equipment_type: payload.equipment_type.clone(), priority: payload.priority, heating_capacity_w: payload.heating_capacity_w, cooling_capacity_w: payload.cooling_capacity_w });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
