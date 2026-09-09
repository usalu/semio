//! 🔺️ Sparse diff builder for `ChangePlantLoopType` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePlantLoopType, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };

    if existing.loop_type == payload.new_loop_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Plant loop {} already has that loop type.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.plant_loops.iter_mut().find(|item| item.id == payload.id) {
        item.loop_type = payload.new_loop_type;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
