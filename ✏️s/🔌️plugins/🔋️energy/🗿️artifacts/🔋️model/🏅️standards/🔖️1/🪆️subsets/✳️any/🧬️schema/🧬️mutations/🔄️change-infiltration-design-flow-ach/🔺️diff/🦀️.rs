//! 🔺️ Sparse diff builder for `ChangeInfiltrationDesignFlowAch` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationDesignFlowAch, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_design_flow_ach.is_finite() || payload.new_design_flow_ach < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Infiltration {}: design flow (air changes per hour) must be a finite non-negative value, got {}.", payload.id.0, payload.new_design_flow_ach), [payload.id.0.to_string()]);
    }
    if existing.design_flow_ach == payload.new_design_flow_ach {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this design flow (air changes per hour): {}.", payload.id.0, payload.new_design_flow_ach));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.design_flow_ach = payload.new_design_flow_ach;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
