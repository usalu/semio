//! 🔺️ Sparse diff builder for `ChangePlantLoopDesignFlow` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePlantLoopDesignFlow, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.plant_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Plant loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_design_flow_kg_s.is_finite() || payload.new_design_flow_kg_s <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A design mass flow must be a positive finite number, got {}.", payload.new_design_flow_kg_s), [payload.id.0.to_string()]);
    }
    if existing.design_flow_kg_s == payload.new_design_flow_kg_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Plant loop {} already has that design mass flow.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.plant_loops.iter_mut().find(|item| item.id == payload.id) {
        item.design_flow_kg_s = payload.new_design_flow_kg_s;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
