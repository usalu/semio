//! 🔺️ Sparse diff builder for `ChangeAirLoopDesignSupplyAirFlow` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeAirLoopDesignSupplyAirFlow, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.air_loops.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Air loop {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_design_supply_air_flow_m3_s.is_finite() || payload.new_design_supply_air_flow_m3_s <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("A design supply air flow must be a positive finite number, got {}.", payload.new_design_supply_air_flow_m3_s), [payload.id.0.to_string()]);
    }
    if existing.design_supply_air_flow_m3_s == payload.new_design_supply_air_flow_m3_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Air loop {} already has that design supply air flow.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.air_loops.iter_mut().find(|item| item.id == payload.id) {
        item.design_supply_air_flow_m3_s = payload.new_design_supply_air_flow_m3_s;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
