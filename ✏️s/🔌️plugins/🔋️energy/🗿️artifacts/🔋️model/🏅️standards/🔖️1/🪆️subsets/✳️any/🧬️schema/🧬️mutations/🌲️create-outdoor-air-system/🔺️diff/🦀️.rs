//! 🔺️ Sparse diff builder for `CreateOutdoorAirSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateOutdoorAirSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.outdoor_air_systems.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Outdoor air system {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.air_loops.iter().any(|item| item.id == payload.air_loop_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.air_loop_id.0), [payload.air_loop_id.0.to_string()]);
    }
    if !payload.min_oa_flow_m3_s.is_finite() || payload.min_oa_flow_m3_s < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A minimum outdoor air flow must be a non-negative finite number, got {}.", payload.min_oa_flow_m3_s), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.outdoor_air_systems.push(crate::model::OutdoorAirSystem { id: payload.id, air_loop_id: payload.air_loop_id, min_oa_flow_m3_s: payload.min_oa_flow_m3_s, economizer_enabled: payload.economizer_enabled });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
