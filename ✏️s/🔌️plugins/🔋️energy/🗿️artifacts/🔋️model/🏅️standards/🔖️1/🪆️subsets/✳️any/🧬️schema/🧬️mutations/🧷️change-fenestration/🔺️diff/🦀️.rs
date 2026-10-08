//! 🔺️ Sparse diff builder for `ChangeFenestrationDividerConductance` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FenestrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationDividerConductance, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_divider_conductance_w_k.is_finite() || payload.new_divider_conductance_w_k < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fenestration {} needs a non-negative finite divider conductance, got {}.", payload.id.0, payload.new_divider_conductance_w_k), [payload.id.0.to_string()]);
    }
    if existing.divider_conductance_w_k == payload.new_divider_conductance_w_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fenestration {} already has this divider conductance.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { fenestrations: Rows::modifying(FenestrationPatch { divider_conductance_w_k: Some(payload.new_divider_conductance_w_k), ..FenestrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
