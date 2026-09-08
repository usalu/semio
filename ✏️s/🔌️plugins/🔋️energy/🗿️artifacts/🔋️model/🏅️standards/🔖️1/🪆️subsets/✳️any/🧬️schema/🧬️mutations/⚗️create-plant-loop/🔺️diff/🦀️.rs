//! 🔺️ Sparse diff builder for `CreatePlantLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreatePlantLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.plant_loops.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Plant loop {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "A plant loop name must not be blank.".to_string(), [payload.id.0.to_string()]);
    }
    if base.model.plant_loops.iter().any(|item| item.name == payload.name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another plant loop is already named {:?}.", payload.name), [payload.id.0.to_string()]);
    }
    if !payload.supply_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.supply_temperature_c) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A supply temperature must lie between -100.0 and 300.0, got {}.", payload.supply_temperature_c), [payload.id.0.to_string()]);
    }
    if !payload.return_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.return_temperature_c) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A return temperature must lie between -100.0 and 300.0, got {}.", payload.return_temperature_c), [payload.id.0.to_string()]);
    }
    if !payload.design_flow_kg_s.is_finite() || payload.design_flow_kg_s <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A design mass flow must be a positive finite number, got {}.", payload.design_flow_kg_s), [payload.id.0.to_string()]);
    }
    if payload.equipment_ids.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return protocol::MutationOutcome::error("mutation.invariant", "A plant equipment list is kept ascending and free of duplicates.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.equipment_ids.iter().any(|entry| entry.0 == 0) {
        return protocol::MutationOutcome::error("mutation.invariant", "A plant equipment list carries no unset id.".to_string(), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.plant_loops.push(crate::model::PlantLoopConfig { id: payload.id, name: payload.name.clone(), loop_type: payload.loop_type, supply_temperature_c: payload.supply_temperature_c, return_temperature_c: payload.return_temperature_c, design_flow_kg_s: payload.design_flow_kg_s, equipment_ids: payload.equipment_ids.clone() });
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
