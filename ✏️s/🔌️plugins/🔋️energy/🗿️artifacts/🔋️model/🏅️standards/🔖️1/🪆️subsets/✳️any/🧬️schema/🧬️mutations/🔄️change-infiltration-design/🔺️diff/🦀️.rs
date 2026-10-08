//! 🔺️ Sparse diff builder for `ChangeInfiltrationDesignFlowAch` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, InfiltrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationDesignFlowAch, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_design_flow_ach.is_finite() || payload.new_design_flow_ach < 0.0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Infiltration {}: design flow (air changes per hour) must be a finite non-negative value, got {}.", payload.id.0, payload.new_design_flow_ach),
            [payload.id.0.to_string()],
        );
    }
    if existing.design_flow_ach == payload.new_design_flow_ach {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Infiltration {} already carries this design flow (air changes per hour): {}.", payload.id.0, payload.new_design_flow_ach));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { infiltrations: Rows::modifying(InfiltrationPatch { design_flow_ach: Some(payload.new_design_flow_ach), ..InfiltrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
