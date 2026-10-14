//! 🔺️ Sparse diff builder for `ChangeZoneEquipmentType` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, ZoneEquipmentAssignmentPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeZoneEquipmentType, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.zone_equipment.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone equipment {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };

    if existing.equipment_type == payload.new_equipment_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Zone equipment {} already has that equipment type.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { zone_equipment: Rows::modifying(ZoneEquipmentAssignmentPatch { equipment_type: Some(payload.new_equipment_type.clone()), ..ZoneEquipmentAssignmentPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
