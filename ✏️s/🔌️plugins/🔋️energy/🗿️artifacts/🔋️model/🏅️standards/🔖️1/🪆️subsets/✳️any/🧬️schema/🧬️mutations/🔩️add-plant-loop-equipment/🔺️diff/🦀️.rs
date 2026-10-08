//! 🔺️ Sparse diff builder for `AddPlantLoopEquipment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ListEdit, ModelPatch, PlantLoopConfigPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddPlantLoopEquipment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.equipment_id.0 == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Plant equipment zero is the unset id, not a reference.".to_string(), [payload.equipment_id.0.to_string()]);
    }
    if existing.equipment_ids.contains(&payload.equipment_id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Plant loop {} already lists plant equipment {}.", payload.id.0, payload.equipment_id.0), [payload.equipment_id.0.to_string()]);
    }
    let position = payload.index.map_or_else(|| existing.equipment_ids.iter().position(|entry| entry.0 > payload.equipment_id.0).unwrap_or(existing.equipment_ids.len()), |index| index as usize);
    if position > existing.equipment_ids.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the list of {} equipment_ids.", position, existing.equipment_ids.len()), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { plant_loops: Rows::modifying(PlantLoopConfigPatch { equipment_ids: ListEdit::inserting(position, payload.equipment_id), ..PlantLoopConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
