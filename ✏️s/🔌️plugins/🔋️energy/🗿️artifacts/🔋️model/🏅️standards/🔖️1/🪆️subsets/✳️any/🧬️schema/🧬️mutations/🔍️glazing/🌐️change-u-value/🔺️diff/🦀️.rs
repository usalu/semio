//! 🔺️ Sparse diff builder for `ChangeFenestrationUValue` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FenestrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationUValue, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_u_value_w_m2k.is_finite() || payload.new_u_value_w_m2k <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fenestration {} needs a positive finite U-value, got {}.", payload.id.0, payload.new_u_value_w_m2k), [payload.id.0.to_string()]);
    }
    if existing.u_value_w_m2k == payload.new_u_value_w_m2k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fenestration {} already has this U-value.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { fenestrations: Rows::modifying(FenestrationPatch { u_value_w_m2k: Some(payload.new_u_value_w_m2k), ..FenestrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
