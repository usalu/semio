//! 🔺️ Sparse diff builder for `RemovePlantLoopEquipment` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::RemovePlantLoopEquipment, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.equipment_ids.contains(&payload.equipment_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not list plant equipment {}.", payload.id.0, payload.equipment_id.0), [payload.equipment_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.plant_loops.iter_mut().find(|item| item.id == payload.id) {
        item.equipment_ids.retain(|entry| *entry != payload.equipment_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
