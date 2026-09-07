//! ↩️ Inverse for `CreateAirLoop` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateAirLoop, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if (base.model.air_loops.iter().any(|item| item.id == payload.id)) || (payload.name.trim().is_empty()) || (base.model.air_loops.iter().any(|item| item.name == payload.name)) || (payload.supply_node_id == 0) || (payload.return_node_id == 0) || (!payload.design_supply_air_flow_m3_s.is_finite() || payload.design_supply_air_flow_m3_s <= 0.0) || (payload.terminal_zone_ids.windows(2).any(|pair| pair[0].0 >= pair[1].0)) || (payload.terminal_zone_ids.iter().any(|entry| !base.model.zones.iter().any(|zone| zone.id == *entry))) {
        return Vec::new();
    }
    vec![vocabulary::delete_air_loop(payload.id)]
}
//#endregion 🔖️Inverse
