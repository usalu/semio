//! 🔺️ Sparse diff builder for `ChangeOutdoorAirSystemMinOaFlow` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeOutdoorAirSystemMinOaFlow, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.outdoor_air_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Outdoor air system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_min_oa_flow_m3_s.is_finite() || payload.new_min_oa_flow_m3_s < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A minimum outdoor air flow must be a non-negative finite number, got {}.", payload.new_min_oa_flow_m3_s), [payload.id.0.to_string()]);
    }
    if existing.min_oa_flow_m3_s == payload.new_min_oa_flow_m3_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Outdoor air system {} already has that minimum outdoor air flow.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.outdoor_air_systems.iter_mut().find(|item| item.id == payload.id) {
        item.min_oa_flow_m3_s = payload.new_min_oa_flow_m3_s;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
