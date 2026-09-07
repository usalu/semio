//! 🔺️ Sparse diff builder for `CreateAirLoop` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateAirLoop, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.air_loops.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Air loop {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An air loop name must not be blank.".to_string(), [payload.id.0.to_string()]);
    }
    if base.model.air_loops.iter().any(|item| item.name == payload.name) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Another air loop is already named {:?}.", payload.name), [payload.id.0.to_string()]);
    }
    if payload.supply_node_id == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "A supply node id must be at least one.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.return_node_id == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", "A return node id must be at least one.".to_string(), [payload.id.0.to_string()]);
    }
    if !payload.design_supply_air_flow_m3_s.is_finite() || payload.design_supply_air_flow_m3_s <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A design supply air flow must be a positive finite number, got {}.", payload.design_supply_air_flow_m3_s), [payload.id.0.to_string()]);
    }
    if payload.terminal_zone_ids.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return protocol::MutationOutcome::error("mutation.invariant", "A terminal zone list is kept ascending and free of duplicates.".to_string(), [payload.id.0.to_string()]);
    }
    if payload.terminal_zone_ids.iter().any(|entry| !base.model.zones.iter().any(|zone| zone.id == *entry)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "A terminal zone list names a zone this model does not have.".to_string(), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.air_loops.push(crate::model::ModelAirLoop { id: payload.id, name: payload.name.clone(), supply_node_id: payload.supply_node_id, return_node_id: payload.return_node_id, design_supply_air_flow_m3_s: payload.design_supply_air_flow_m3_s, terminal_zone_ids: payload.terminal_zone_ids.clone() });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
