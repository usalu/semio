//! 🔺️ Sparse diff builder for `CreateOutdoorAirSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, OutdoorAirSystemPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateOutdoorAirSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.outdoor_air_systems.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Outdoor air system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.air_loops.iter().any(|item| item.id == payload.air_loop_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.air_loop_id.0), [payload.air_loop_id.0.to_string()]);
    }
    if !payload.min_oa_flow_m3_s.is_finite() || payload.min_oa_flow_m3_s < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A minimum outdoor air flow must be a non-negative finite number, got {}.", payload.min_oa_flow_m3_s), [payload.id.0.to_string()]);
    }
    let position = payload.index.map_or(base.model.outdoor_air_systems.len(), |index| index as usize);
    if position > base.model.outdoor_air_systems.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} outdoor_air_systems.", position, base.model.outdoor_air_systems.len()), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { outdoor_air_systems: Rows::inserting(position, crate::model::OutdoorAirSystem { id: payload.id, air_loop_id: payload.air_loop_id, min_oa_flow_m3_s: payload.min_oa_flow_m3_s, economizer_enabled: payload.economizer_enabled }), ..Default::default() }))
}
//#endregion 🔖️Diff
